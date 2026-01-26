package main

import (
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// --- Package Model ---
type Package struct {
	Name        string
	Description string
	Version     string
	Installed   bool
	Installing  bool
	Progress    float64
}

// --- Package Manager Model ---
type PkgMgrModel struct {
	Packages   []Package
	Cursor     int
	Installing bool
}

func InitialPkgMgr() PkgMgrModel {
	return PkgMgrModel{
		Packages: []Package{
			{"NeuralMesh Core", "Decentralized AI Swarm Protocol", "v3.2.1", true, false, 0},
			{"QuantumKernel", "L5 NIST PQC Cryptography Engine", "v1.2.0", false, false, 0},
			{"BioSeal SDK", "Biometric Identity Layer", "v0.9.5-beta", false, false, 0},
			{"Vortex Sentinel", "Real-time Threat Monitoring", "v2.0.0", true, false, 0},
			{"Chaos Engine", "Entropy Generator & LLE Probe", "v4.1.0", false, false, 0},
		},
		Cursor: 0,
	}
}

// --- Update Logic ---
func (m PkgMgrModel) Update(msg tea.Msg) (PkgMgrModel, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "up", "k":
			if m.Cursor > 0 {
				m.Cursor--
			}
		case "down", "j":
			if m.Cursor < len(m.Packages)-1 {
				m.Cursor++
			}
		case "enter", " ":
			// Toggle Install Simulation
			idx := m.Cursor
			if !m.Packages[idx].Installed && !m.Packages[idx].Installing {
				m.Packages[idx].Installing = true
				return m, installCmd(idx)
			}
		}

	case InstallProgressMsg:
		// Update progress
		if m.Packages[msg.Index].Installing {
			m.Packages[msg.Index].Progress += 0.1
			if m.Packages[msg.Index].Progress >= 1.0 {
				m.Packages[msg.Index].Installing = false
				m.Packages[msg.Index].Installed = true
				m.Packages[msg.Index].Progress = 0
			} else {
				// Continue tick
				return m, installTick(msg.Index)
			}
		}
	}
	return m, nil
}

// --- Commands ---
type InstallProgressMsg struct{ Index int }

func installCmd(index int) tea.Cmd {
	return installTick(index)
}

func installTick(index int) tea.Cmd {
	return tea.Tick(time.Millisecond*200, func(t time.Time) tea.Msg {
		return InstallProgressMsg{Index: index}
	})
}

// --- View Logic ---
func (m PkgMgrModel) View() string {
	s := TitleStyle.Render("AVAILABLE PACKAGES") + "\n\n"

	for i, pkg := range m.Packages {
		cursor := " " // no cursor
		if m.Cursor == i {
			cursor = ">" // cursor
		}

		// Icon / Status
		status := "[ ]"
		style := lipgloss.NewStyle().Foreground(SecondaryColor)

		if pkg.Installed {
			status = "[✓]"
			style = style.Foreground(lipgloss.Color("#22c55e"))
		} else if pkg.Installing {
			// Progress Bar
			bars := int(pkg.Progress * 10)
			barStr := ""
			for b := 0; b < bars; b++ {
				barStr += "█"
			}
			for b := bars; b < 10; b++ {
				barStr += "░"
			}
			status = "[" + barStr + "]"
			style = style.Foreground(PrimaryColor)
		} else {
			style = style.Foreground(lipgloss.Color("#DDD")) // Available
		}

		// Render Line
		prefix := lipgloss.NewStyle().Foreground(PrimaryColor).Render(cursor)
		name := style.Bold(true).Render(pkg.Name)
		version := lipgloss.NewStyle().Foreground(SecondaryColor).Italic(true).Render(pkg.Version)
		desc := lipgloss.NewStyle().Foreground(lipgloss.Color("#666")).Render(pkg.Description)

		line := fmt.Sprintf("%s %s %s   %s\n    %s", prefix, status, name, version, desc)

		if m.Cursor == i {
			// Highlight Active Row BG slightly
			line = lipgloss.NewStyle().
				Border(lipgloss.NormalBorder(), false, false, false, true).
				BorderForeground(PrimaryColor).
				PaddingLeft(1).
				Render(line)
		}

		s += line + "\n\n"
	}

	hint := SubTitleStyle.Render("Use ↑/↓ to navigate, ENTER to install/update.")

	return ContainerStyle.Width(65).Render(s + "\n" + hint)
}
