package main

import (
	"fmt"
	"math/rand"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

// --- Dashboard Model ---
type DashboardModel struct {
	Uptime       time.Duration
	Lyapunov     float64
	EntropyRate  float64
	HealthStatus string
	Ticks        int
}

func InitialDashboard() DashboardModel {
	return DashboardModel{
		Uptime:       0,
		Lyapunov:     0.0,
		EntropyRate:  0.0,
		HealthStatus: "HEALTHY",
		Ticks:        0,
	}
}

// --- Update Logic ---
func (m DashboardModel) Update(msg tea.Msg) (DashboardModel, tea.Cmd) {
	switch msg.(type) {
	case TickMsg:
		m.Ticks++
		m.Uptime += time.Second

		// Simulate dynamic chaotic data
		m.Lyapunov = 0.05 + (rand.Float64() * 0.02)   // fluctuate around 0.05
		m.EntropyRate = 1200 + (rand.Float64() * 500) // fluctuate speed

		if rand.Intn(100) > 95 {
			m.HealthStatus = "MARGINAL"
		} else {
			m.HealthStatus = "HEALTHY"
		}
		return m, nil
	}
	return m, nil
}

// --- View Logic ---
func (m DashboardModel) View() string {
	// Status Block
	statusColor := lipgloss.Color("#22c55e") // Green
	if m.HealthStatus != "HEALTHY" {
		statusColor = lipgloss.Color("#fbbf24") // Yellow
	}

	statusBlock := lipgloss.NewStyle().
		Foreground(statusColor).
		Border(lipgloss.RoundedBorder()).
		BorderForeground(statusColor).
		Padding(0, 1).
		Render(m.HealthStatus)

	// Metrics
	uptime := fmt.Sprintf("Uptime: %s", m.Uptime.Round(time.Second))
	lya := fmt.Sprintf("Lyapunov Exp: %.6f", m.Lyapunov)
	ent := fmt.Sprintf("Entropy Rate: %.2f MB/s", m.EntropyRate)

	// Visual Bar for LLE
	barW := 20
	filled := int((m.Lyapunov / 0.10) * float64(barW))
	if filled > barW {
		filled = barW
	}
	bar := ""
	for i := 0; i < filled; i++ {
		bar += "█"
	}
	for i := filled; i < barW; i++ {
		bar += "░"
	}

	barChart := lipgloss.NewStyle().Foreground(PrimaryColor).Render(bar)

	// Composition
	col1 := lipgloss.JoinVertical(lipgloss.Left,
		TitleStyle.Render("SYSTEM STATUS"),
		statusBlock,
		"",
		SubTitleStyle.Render("System Metrics"),
		uptime,
	)

	col2 := lipgloss.JoinVertical(lipgloss.Left,
		TitleStyle.Render("CHAOS ENGINE"),
		lya,
		barChart,
		"",
		ent,
	)

	return lipgloss.JoinHorizontal(lipgloss.Top,
		ContainerStyle.Width(30).Render(col1),
		ContainerStyle.Width(40).Render(col2),
	)
}
