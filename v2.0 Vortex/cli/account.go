package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"path/filepath"
	"time"
)

// --- User Account Types ---
type UserCredentials struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

type UserSession struct {
	Token        string    `json:"token"`
	RefreshToken string    `json:"refresh_token"`
	ExpiresAt    time.Time `json:"expires_at"`
	Email        string    `json:"email"`
}

type RegistrationResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
	UserID  string `json:"user_id"`
}

type LoginResponse struct {
	Success      bool   `json:"success"`
	Token        string `json:"token"`
	RefreshToken string `json:"refresh_token"`
	ExpiresIn    int    `json:"expires_in"` // seconds
}

// --- Session Management ---
const SessionFile = ".vortex_session"

func getSessionPath() string {
	home, _ := os.UserHomeDir()
	return filepath.Join(home, SessionFile)
}

func SaveSession(session *UserSession) error {
	data, err := json.MarshalIndent(session, "", "  ")
	if err != nil {
		return err
	}
	return ioutil.WriteFile(getSessionPath(), data, 0600)
}

func LoadSession() (*UserSession, error) {
	path := getSessionPath()
	data, err := ioutil.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil // No session
		}
		return nil, err
	}

	var session UserSession
	if err := json.Unmarshal(data, &session); err != nil {
		return nil, err
	}

	// Check if expired
	if time.Now().After(session.ExpiresAt) {
		return nil, fmt.Errorf("session expired")
	}

	return &session, nil
}

func ClearSession() error {
	return os.Remove(getSessionPath())
}

// --- API Endpoints ---
func RegisterUser(email, password string) (*RegistrationResponse, error) {
	creds := UserCredentials{
		Email:    email,
		Password: password,
	}

	jsonData, err := json.Marshal(creds)
	if err != nil {
		return nil, err
	}

	resp, err := http.Post(
		RepoBaseURL+"/api/register",
		"application/json",
		bytes.NewBuffer(jsonData),
	)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var result RegistrationResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, err
	}

	return &result, nil
}

func LoginUser(email, password string) (*UserSession, error) {
	creds := UserCredentials{
		Email:    email,
		Password: password,
	}

	jsonData, err := json.Marshal(creds)
	if err != nil {
		return nil, err
	}

	resp, err := http.Post(
		RepoBaseURL+"/api/login",
		"application/json",
		bytes.NewBuffer(jsonData),
	)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return nil, fmt.Errorf("login failed: invalid credentials")
	}

	var result LoginResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, err
	}

	if !result.Success {
		return nil, fmt.Errorf("login failed")
	}

	// Create session
	session := &UserSession{
		Token:        result.Token,
		RefreshToken: result.RefreshToken,
		ExpiresAt:    time.Now().Add(time.Duration(result.ExpiresIn) * time.Second),
		Email:        email,
	}

	return session, nil
}

func RefreshSession(refreshToken string) (*UserSession, error) {
	data := map[string]string{"refresh_token": refreshToken}
	jsonData, err := json.Marshal(data)
	if err != nil {
		return nil, err
	}

	resp, err := http.Post(
		RepoBaseURL+"/api/refresh",
		"application/json",
		bytes.NewBuffer(jsonData),
	)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	var result LoginResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return nil, err
	}

	session := &UserSession{
		Token:        result.Token,
		RefreshToken: result.RefreshToken,
		ExpiresAt:    time.Now().Add(time.Duration(result.ExpiresIn) * time.Second),
	}

	return session, nil
}

// --- Authenticated Client (Updated for User Sessions) ---
func NewSessionClient(session *UserSession) *http.Client {
	return &http.Client{
		Transport: &SessionTransport{
			Session:   session,
			Transport: http.DefaultTransport,
		},
	}
}

type SessionTransport struct {
	Session   *UserSession
	Transport http.RoundTripper
}

func (t *SessionTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	req.Header.Set("Authorization", "Bearer "+t.Session.Token)
	req.Header.Set("User-Agent", "Vortex-CLI/2.0.1")
	return t.Transport.RoundTrip(req)
}
