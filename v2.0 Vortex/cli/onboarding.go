package main

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// --- Onboarding Model ---
type OnboardingModel struct {
	Step            int
	AuthMode        string // "none", "login", "register", "apikey"
	Email           string
	Password        string
	PasswordConfirm string
	APIKey          string
	InstallMode     int // 0=Auto, 1=Custom, 2=Download Only
	SelectedLibs    map[string]bool
	Installing      bool
	Progress        float64
	CurrentDownload string
	ErrorMsg        string
	Session         *UserSession
}

func InitialOnboarding() OnboardingModel {
	return OnboardingModel{
		Step:     0,
		AuthMode: "none",
		SelectedLibs: map[string]bool{
			"core":       true, // Required
			"neuralmesh": false,
			"bioseal":    false,
			"quantum":    false,
		},
	}
}

// --- Update Logic ---
func (m OnboardingModel) Update(msg tea.Msg) (OnboardingModel, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		// Step 0: Auth method selection
		case "1":
			if m.Step == 0 {
				m.AuthMode = "login"
				m.Step = 1 // Go to login
			} else if m.Step == 2 {
				m.InstallMode = 0 // Auto Install
				m.Step = 4        // Skip to installation
				return m, startInstall()
			}
		case "2":
			if m.Step == 0 {
				m.AuthMode = "register"
				m.Step = 1 // Go to registration
			} else if m.Step == 2 {
				m.InstallMode = 1 // Custom
				m.Step = 3        // Go to library selection
			}
		case "3":
			if m.Step == 0 {
				m.AuthMode = "apikey"
				m.Step = 1 // Go to API key input
			} else if m.Step == 2 {
				m.InstallMode = 2 // Download Only
				m.Step = 3
			}
		case "enter":
			if m.Step == 1 {
				// Handle authentication
				if m.AuthMode == "login" {
					// Simulate login
					session := &UserSession{
						Token: "simulated_token",
						Email: m.Email,
					}
					m.Session = session
					m.Step = 2 // Go to installation type selection
				} else if m.AuthMode == "register" {
					// Simulate registration
					m.Step = 2
				} else if m.AuthMode == "apikey" {
					// Validate API key
					m.Step = 2
				}
			} else if m.Step == 3 {
				m.Step = 4
				return m, startInstall()
			}
		}
	case InstallProgressMsg:
		m.Progress += 0.05
		if m.Progress >= 1.0 {
			m.Installing = false
			m.Progress = 0
			m.Step = 5 // Complete
		} else {
			return m, installTick(0)
		}
	}
	return m, nil
}

func startInstall() tea.Cmd {
	return installTick(0)
}

// --- View Logic ---
func (m OnboardingModel) View() string {
	switch m.Step {
	case 0:
		return m.renderAuthSelection()
	case 1:
		if m.AuthMode == "login" {
			return m.renderLogin()
		} else if m.AuthMode == "register" {
			return m.renderRegister()
		} else if m.AuthMode == "apikey" {
			return m.renderAPIKey()
		}
	case 2:
		return m.renderInstallTypeSelection()
	case 3:
		return m.renderLibrarySelection()
	case 4:
		return m.renderInstallation()
	case 5:
		return m.renderComplete()
	}
	return ""
}

func (m OnboardingModel) renderAuthSelection() string {
	title := RenderGradientText("WELCOME TO VORTEX v2.0")

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		SubTitleStyle.Render("Authentication Required:"),
		"",
		"  [1] Login to Existing Account",
		"  [2] Create New Account",
		"  [3] Use API Key (Advanced)",
		"",
		SubTitleStyle.Render("Press 1, 2, or 3 to continue"),
	)

	return ContainerStyle.Width(60).Render(content)
}

func (m OnboardingModel) renderLogin() string {
	title := TitleStyle.Render("LOGIN TO VORTEX")

	emailField := fmt.Sprintf("Email: %s_", m.Email)
	passField := "Password: " + lipgloss.NewStyle().Foreground(SecondaryColor).Render("••••••••")

	var errorDisplay string
	if m.ErrorMsg != "" {
		errorDisplay = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#ef4444")).
			Render("⚠ " + m.ErrorMsg)
	}

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		emailField,
		passField,
		"",
		errorDisplay,
		"",
		SubTitleStyle.Render("Press ENTER to login"),
	)

	return ContainerStyle.Width(60).Render(content)
}

func (m OnboardingModel) renderRegister() string {
	title := TitleStyle.Render("CREATE VORTEX ACCOUNT")

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		fmt.Sprintf("Email: %s_", m.Email),
		"Password: ••••••••",
		"Confirm: ••••••••",
		"",
		SubTitleStyle.Render("Press ENTER to create account"),
	)

	return ContainerStyle.Width(60).Render(content)
}

func (m OnboardingModel) renderAPIKey() string {
	title := TitleStyle.Render("API KEY AUTHENTICATION")

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		"Enter your Vortex API key:",
		"",
		fmt.Sprintf("Key: %s_", m.APIKey),
		"",
		SubTitleStyle.Render("Press ENTER to validate"),
	)

	return ContainerStyle.Width(60).Render(content)
}

func (m OnboardingModel) renderInstallTypeSelection() string {
	title := TitleStyle.Render("SELECT INSTALLATION TYPE")

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		"  [1] Quick Start (Auto Install All)",
		"  [2] Custom (Select Components)",
		"  [3] Download Only (Manual Install)",
		"",
		SubTitleStyle.Render("Press 1, 2, or 3 to continue"),
	)

	return ContainerStyle.Width(60).Render(content)
}

func (m OnboardingModel) renderLibrarySelection() string {
	title := TitleStyle.Render("SELECT COMPONENTS")

	libs := []struct {
		key      string
		name     string
		desc     string
		required bool
	}{
		{"core", "Core Library", "QST HyperCycle Vortex (Required)", true},
		{"neuralmesh", "NeuralMesh", "Decentralized AI Protocol", false},
		{"bioseal", "BioSeal SDK", "Biometric Identity Layer", false},
		{"quantum", "Quantum Kernel", "Extended PQC Algorithms", false},
	}

	var items []string
	for _, lib := range libs {
		checked := "[ ]"
		if m.SelectedLibs[lib.key] {
			checked = "[✓]"
		}
		if lib.required {
			checked = "[✓]"
		}

		line := fmt.Sprintf("  %s %s\n      %s",
			checked,
			lipgloss.NewStyle().Bold(true).Render(lib.name),
			SubTitleStyle.Render(lib.desc))
		items = append(items, line)
	}

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		lipgloss.JoinVertical(lipgloss.Left, items...),
		"",
		SubTitleStyle.Render("Press ENTER to continue"),
	)

	return ContainerStyle.Width(65).Render(content)
}

func (m OnboardingModel) renderInstallation() string {
	title := TitleStyle.Render("INSTALLING LIBRARIES")

	// Progress bar
	barW := 50
	filled := int(m.Progress * float64(barW))
	bar := ""
	for i := 0; i < filled; i++ {
		bar += "█"
	}
	for i := filled; i < barW; i++ {
		bar += "░"
	}

	barRender := lipgloss.NewStyle().Foreground(PrimaryColor).Render(bar)
	percent := fmt.Sprintf("%.0f%%", m.Progress*100)

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		"Downloading and installing components...",
		"",
		barRender,
		percent,
	)

	return ContainerStyle.Width(60).Render(content)
}

func (m OnboardingModel) renderComplete() string {
	title := RenderGradientText("INSTALLATION COMPLETE")

	content := lipgloss.JoinVertical(lipgloss.Left,
		title,
		"",
		lipgloss.NewStyle().Foreground(lipgloss.Color("#22c55e")).Render("✓ All components installed successfully"),
		"",
		"Launch Vortex Dashboard with:",
		lipgloss.NewStyle().Foreground(PrimaryColor).Render("  vortex dash"),
		"",
		SubTitleStyle.Render("Press Q to exit"),
	)

	return ContainerStyle.Width(60).Render(content)
}
