package main

import (
	"github.com/charmbracelet/lipgloss"
)

// Mock Settings (Read Only for MVP)
var (
	CurrentTheme    = "Orange Gradient (Active)"
	PathConfig      = "C:\\Program Files\\HyperCycle"
	UpdateChannel   = "Stable"
	TelemetryUpload = "Enabled"
)

func RenderSettings() string {
	itemStyle := lipgloss.NewStyle().
		PaddingLeft(2).
		Foreground(lipgloss.Color("#DDD"))

	labelStyle := lipgloss.NewStyle().
		Foreground(SecondaryColor).
		Width(20)

	row := func(label, value string) string {
		return lipgloss.JoinHorizontal(lipgloss.Left,
			labelStyle.Render(label),
			itemStyle.Render(value),
		)
	}

	content := lipgloss.JoinVertical(lipgloss.Left,
		TitleStyle.Render("CONFIGURATION"),
		"",
		row("Theme", CurrentTheme),
		row("Install Path", PathConfig),
		row("Update Channel", UpdateChannel),
		row("Telemetry", TelemetryUpload),
		"",
		SubTitleStyle.Render("Edit config.yaml to change these values."),
	)

	return ContainerStyle.Width(60).Render(content)
}
