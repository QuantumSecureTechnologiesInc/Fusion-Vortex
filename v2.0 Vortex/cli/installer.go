package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"runtime"
)

// --- Library Types ---
type Library struct {
	ID          string
	Name        string
	Description string
	Version     string
	Required    bool
	Platforms   map[string]PlatformBinary
}

type PlatformBinary struct {
	URL      string
	Checksum string
	FileName string
}

// --- Library Catalog (loaded from private repo) ---
func LoadLibraryCatalog(apiKey string) ([]Library, error) {
	manifest, err := FetchManifest(apiKey)
	if err != nil {
		return nil, err
	}

	var libraries []Library
	for _, pkg := range manifest.Packages {
		platforms := make(map[string]PlatformBinary)
		for platform, info := range pkg.Platforms {
			platforms[platform] = PlatformBinary{
				URL:      info.URL,
				Checksum: info.Checksum,
				FileName: info.FileName,
			}
		}

		libraries = append(libraries, Library{
			ID:          pkg.ID,
			Name:        pkg.Name,
			Description: pkg.Description,
			Version:     pkg.Version,
			Required:    pkg.ID == "core",
			Platforms:   platforms,
		})
	}

	return libraries, nil
}

// --- Installer Functions ---

func GetPlatform() string {
	return runtime.GOOS // "windows", "linux", "darwin"
}

func GetInstallPath() string {
	switch GetPlatform() {
	case "windows":
		return filepath.Join(os.Getenv("ProgramFiles"), "HyperCycle", "lib")
	case "linux":
		return "/usr/local/lib/hypercycle"
	case "darwin":
		return "/usr/local/lib/hypercycle"
	default:
		return "./lib"
	}
}

func DownloadLibrary(lib Library, apiKey string, progressChan chan float64) error {
	platform := GetPlatform()
	binary, ok := lib.Platforms[platform]
	if !ok {
		return fmt.Errorf("platform %s not supported for %s", platform, lib.Name)
	}

	// Create install directory
	installPath := GetInstallPath()
	os.MkdirAll(installPath, 0755)

	destPath := filepath.Join(installPath, binary.FileName)

	// Create authenticated client
	client := NewAuthenticatedClient(apiKey)

	// Download file
	resp, err := client.Get(binary.URL)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return fmt.Errorf("download failed: status %d", resp.StatusCode)
	}

	out, err := os.Create(destPath)
	if err != nil {
		return err
	}
	defer out.Close()

	// Progress tracking
	totalSize := resp.ContentLength
	downloaded := int64(0)

	buf := make([]byte, 32*1024)
	for {
		n, err := resp.Body.Read(buf)
		if n > 0 {
			out.Write(buf[:n])
			downloaded += int64(n)
			if progressChan != nil && totalSize > 0 {
				progressChan <- float64(downloaded) / float64(totalSize)
			}
		}
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}
	}

	// Verify checksum
	if err := verifyChecksum(destPath, binary.Checksum); err != nil {
		os.Remove(destPath)
		return err
	}

	return nil
}

func verifyChecksum(filePath, expectedChecksum string) error {
	f, err := os.Open(filePath)
	if err != nil {
		return err
	}
	defer f.Close()

	h := sha256.New()
	if _, err := io.Copy(h, f); err != nil {
		return err
	}

	actualChecksum := hex.EncodeToString(h.Sum(nil))
	if actualChecksum != expectedChecksum {
		return fmt.Errorf("checksum mismatch: expected %s, got %s", expectedChecksum, actualChecksum)
	}

	return nil
}

// --- PATH Management ---

func AddToPath() error {
	installPath := GetInstallPath()

	switch GetPlatform() {
	case "windows":
		// TODO: Add registry key modification for PATH
		fmt.Printf("Please add %s to your PATH manually\n", installPath)
	case "linux", "darwin":
		// TODO: Add to ~/.bashrc or ~/.zshrc
		fmt.Printf("Please add %s to your PATH manually\n", installPath)
	}

	return nil
}
