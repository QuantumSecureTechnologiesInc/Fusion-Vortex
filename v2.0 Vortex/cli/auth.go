package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"path/filepath"
)

// --- Authentication ---
const (
	AuthConfigFile = ".vortex_auth"
	RepoBaseURL    = "https://repo.hypercycle.ai"
)

type AuthConfig struct {
	APIKey     string `json:"api_key"`
	LicenseKey string `json:"license_key"`
	Configured bool   `json:"configured"`
}

func getAuthConfigPath() string {
	home, _ := os.UserHomeDir()
	return filepath.Join(home, AuthConfigFile)
}

func LoadAuthConfig() (*AuthConfig, error) {
	path := getAuthConfigPath()
	data, err := ioutil.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return &AuthConfig{}, nil
		}
		return nil, err
	}

	var config AuthConfig
	if err := json.Unmarshal(data, &config); err != nil {
		return nil, err
	}
	return &config, nil
}

func SaveAuthConfig(config *AuthConfig) error {
	path := getAuthConfigPath()
	data, err := json.MarshalIndent(config, "", "  ")
	if err != nil {
		return err
	}
	return ioutil.WriteFile(path, data, 0600)
}

func ValidateAPIKey(apiKey string) (bool, error) {
	req, err := http.NewRequest("GET", RepoBaseURL+"/api/validate", nil)
	if err != nil {
		return false, err
	}
	req.Header.Set("X-API-Key", apiKey)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return false, err
	}
	defer resp.Body.Close()

	return resp.StatusCode == 200, nil
}

// --- Authenticated HTTP Client ---
func NewAuthenticatedClient(apiKey string) *http.Client {
	return &http.Client{
		Transport: &AuthenticatedTransport{
			APIKey:    apiKey,
			Transport: http.DefaultTransport,
		},
	}
}

type AuthenticatedTransport struct {
	APIKey    string
	Transport http.RoundTripper
}

func (t *AuthenticatedTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	req.Header.Set("X-API-Key", t.APIKey)
	req.Header.Set("User-Agent", "Vortex-CLI/2.0.1")
	return t.Transport.RoundTrip(req)
}

// --- Repository Manifest ---
type RepositoryManifest struct {
	Packages []PackageInfo `json:"packages"`
}

type PackageInfo struct {
	ID          string                        `json:"id"`
	Name        string                        `json:"name"`
	Version     string                        `json:"version"`
	Description string                        `json:"description"`
	Platforms   map[string]PlatformBinaryInfo `json:"platforms"`
}

type PlatformBinaryInfo struct {
	URL          string `json:"url"`
	Checksum     string `json:"checksum"`
	FileName     string `json:"filename"`
	AuthRequired bool   `json:"auth_required"`
}

func FetchManifest(apiKey string) (*RepositoryManifest, error) {
	client := NewAuthenticatedClient(apiKey)

	resp, err := client.Get(RepoBaseURL + "/api/packages.json")
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return nil, fmt.Errorf("failed to fetch manifest: status %d", resp.StatusCode)
	}

	var manifest RepositoryManifest
	if err := json.NewDecoder(resp.Body).Decode(&manifest); err != nil {
		return nil, err
	}

	return &manifest, nil
}
