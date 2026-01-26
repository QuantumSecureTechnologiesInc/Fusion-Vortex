package main

import (
	"fmt"
	"os"
	"path/filepath"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// --- Configuration ---
const ConfigFileName = ".vortex_config"

// --- Main Application Model ---
type MainModel struct {
	ActiveView string // "onboarding", "dashboard", "packages", "settings", "help"
	ActiveTab  int
	Dash       DashboardModel
	PkgMgr     PkgMgrModel
	Onboarding OnboardingModel
	FirstRun   bool
	Width      int
	Height     int
}

type TickMsg time.Time

func getConfigPath() string {
	home, _ := os.UserHomeDir()
	return filepath.Join(home, ConfigFileName)
}

func isFirstRun() bool {
	_, err := os.Stat(getConfigPath())
	return os.IsNotExist(err)
}

func markConfigured() error {
	f, err := os.Create(getConfigPath())
	if err != nil {
		return err
	}
	defer f.Close()
	f.WriteString("configured=true\n")
	return nil
}

func InitialMainModel() MainModel {
	firstRun := isFirstRun()
	activeView := "dashboard"
	if firstRun {
		activeView = "onboarding"
	}

	return MainModel{
		ActiveView: activeView,
		ActiveTab:  0,
		Dash:       InitialDashboard(),
		PkgMgr:     InitialPkgMgr(),
		Onboarding: InitialOnboarding(),
		FirstRun:   firstRun,
	}
}

func (m MainModel) Init() tea.Cmd {
	return tea.Tick(time.Second, func(t time.Time) tea.Msg {
		return TickMsg(t)
	})
}

func (m MainModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd
	var cmds []tea.Cmd

	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c":
			return m, tea.Quit
		case "q":
			if m.ActiveView != "onboarding" {
				return m, tea.Quit
			}
		case "tab":
			if m.ActiveView == "dashboard" {
				m.ActiveTab = (m.ActiveTab + 1) % 4
			}
		case "shift+tab":
			if m.ActiveView == "dashboard" {
				m.ActiveTab = (m.ActiveTab - 1 + 4) % 4
			}
		}
	case tea.WindowSizeMsg:
		m.Width = msg.Width
		m.Height = msg.Height
	case TickMsg:
		cmds = append(cmds, tea.Tick(time.Second, func(t time.Time) tea.Msg {
			return TickMsg(t)
		}))
	}

	// Route updates to active view
	if m.ActiveView == "onboarding" {
		m.Onboarding, cmd = m.Onboarding.Update(msg)
		cmds = append(cmds, cmd)

		// Check if onboarding is complete
		if m.Onboarding.Step == 3 {
			// Mark as configured and switch to dashboard
			if msg, ok := msg.(tea.KeyMsg); ok && msg.String() == "q" {
				markConfigured()
				m.ActiveView = "dashboard"
			}
		}
	} else {
		// Dashboard always updates for telemetry
		m.Dash, cmd = m.Dash.Update(msg)
		cmds = append(cmds, cmd)

		// Update package manager if active or installing
		if m.ActiveTab == 1 || m.PkgMgr.Installing {
			m.PkgMgr, cmd = m.PkgMgr.Update(msg)
			cmds = append(cmds, cmd)
		}
	}

	return m, tea.Batch(cmds...)
}

func (m MainModel) View() string {
	if m.ActiveView == "onboarding" {
		return m.Onboarding.View()
	}

	// Render Tabs for Dashboard view
	tabs := []string{"DASHBOARD", "PACKAGES", "SETTINGS", "HELP"}
	var renderedTabs []string

	for i, t := range tabs {
		style := InactiveTabStyle
		if m.ActiveTab == i {
			style = ActiveTabStyle
		}
		renderedTabs = append(renderedTabs, style.Render(t))
	}

	header := lipgloss.JoinHorizontal(lipgloss.Top, renderedTabs...)
	header = lipgloss.NewStyle().MarginBottom(1).Render(header)

	// Render Content based on Active Tab
	var content string
	switch m.ActiveTab {
	case 0:
		content = m.Dash.View()
	case 1:
		content = m.PkgMgr.View()
	case 2:
		content = RenderSettings()
	case 3:
		content = RenderHelp()
	}

	return lipgloss.JoinVertical(lipgloss.Left, header, content)
}

func main() {
	// Check for CLI arguments
	if len(os.Args) > 1 {
		switch os.Args[1] {
		case "dash", "dashboard":
			// Force dashboard mode
			p := tea.NewProgram(InitialMainModel(), tea.WithAltScreen())
			if _, err := p.Run(); err != nil {
				fmt.Printf("Error: %v", err)
				os.Exit(1)
			}
			return
		case "version":
			fmt.Println("Vortex CLI v2.0.1")
			return
		case "help":
			fmt.Println("Vortex CLI - HyperCycle Management Tool")
			fmt.Println("\nUsage:")
			fmt.Println("  vortex          Launch interactive dashboard")
			fmt.Println("  vortex dash     Force dashboard mode")
			fmt.Println("  vortex version  Show version")
			return
		}
	}

	// Launch interactive TUI
	p := tea.NewProgram(InitialMainModel(), tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Error: %v", err)
		os.Exit(1)
	}
}
