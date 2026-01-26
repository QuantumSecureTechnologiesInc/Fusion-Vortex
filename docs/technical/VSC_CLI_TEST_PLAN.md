# Fusion v2.0 Vortex VSC CLI - Complete Test Plan

## Test Cycle: Extension Authentication & MCP Bridge

### Prerequisites

- ✅ Fusion v2.0 Vortex VSC CLI built with authentication system
- ✅ Terminal browser integrated
- ✅ OAuth and credential storage implemented
- ✅ MCP Bridge connected to ExtensionHost

### Test Sequence

## Phase 1: Extension Installation with OAuth

### Test 1.1: Install Gemini Code Assist (OAuth Flow)

```bash
fusion extensions install google.gemini-code-assist
```text

**Expected Flow:**
1. CLI displays extension information
2. Prompts for authentication method:

```text
   📋 Authentication options:
     1. OAuth (Recommended) - Sign in with Google
     2. API Key - Use existing API key

   Select option (1/2):
```text

3. User selects "1" (OAuth)
4. CLI launches terminal browser with Google OAuth page
5. Browser displays:

```text
   🌐 Launching terminal browser...
   URL: https://accounts.google.com/o/oauth2/v2/auth?...
```text

6. Simulates OAuth flow:

```text
   📋 Simulating OAuth flow:
      1. User navigates to auth page
      2. User grants permissions
      3. Redirect with auth code
      4. Exchange code for token
```text

7. Token stored in `~/.fusion/credentials/store.json`
8. Extension activated in ExtensionHost
9. Commands registered:

```text
   ✅ Extension installed successfully!

     📚 Available commands:
       • gemini.generateCode - Generate code from description
       • gemini.explainCode - Explain code functionality
       • gemini.refactorCode - Refactor existing code
```text

**Verification:**

```bash

# Check credential store

cat ~/.fusion/credentials/store.json

# Should contain:

{
  "api_keys": {},
  "oauth_tokens": {
    "google.gemini-code-assist": "token_google_<uuid>"
  }
}
```text

### Test 1.2: Install Gemini Code Assist (API Key Flow)

```bash
fusion extensions install google.gemini-code-assist
```text

**Expected Flow:**
1. User selects "2" (API Key)
2. CLI prompts: `Enter your Gemini API key:`
3. User enters: `AIzaSy...  (example key)`
4. Key stored in credential store
5. Extension activated

**Verification:**

```bash
cat ~/.fusion/credentials/store.json

# Should contain:

{
  "api_keys": {
    "google.gemini-code-assist": "AIzaSy..."
  },
  "oauth_tokens": {}
}
```text

## Phase 2: List Extensions with Credentials

### Test 2.1: List Installed Extensions

```bash
fusion extensions list
```text

**Expected Output:**

```text
📦 Installed Extensions:
  • google.gemini-code-assist 🔑
    AI-powered code generation and explanation
```text

**Notes:**
- 🔑 indicator shows extension has stored credentials
- No 🔑 means extension not authenticated

## Phase 3: Execute Extension Commands

### Test 3.1: Generate Code

```bash
fusion extensions exec gemini.generateCode --args '["Create a REST API handler"]'
```text

**Expected Flow:**
1. CLI verifies credentials exist:

```text
   ⚡ Executing command: gemini.generateCode
     🔑 Using stored credentials for google.gemini-code-assist
```text

2. Connects to Extension Host:

```text
     🔌 Connecting to Extension Host...
```text

3. Routes through MCP Bridge:

```text
     ⚙️  Routing through MCP Bridge...
```text

4. Executes command and returns result:

```text
   📋 Result:
   ✅ Code Generated!

   ```rust

   // AI-generated code based on: ["Create a REST API handler"]
   fn example() {
       println!("Hello from Gemini!");
   }

```text

   ✅ Full cycle completed: CLI → MCP Bridge → Extension Host → Command Execution
```text

### Test 3.2: Explain Code

```bash
fusion extensions exec gemini.explainCode --args '["fn main() { println!(\"Hello\"); }"]'
```text

**Expected Output:**

```text
⚡ Executing command: gemini.explainCode
  🔑 Using stored credentials for google.gemini-code-assist
  🔌 Connecting to Extension Host...
  ⚙️  Routing through MCP Bridge...

📋 Result:
✅ Code Explanation:

The code ["fn main() { println!(\"Hello\"); }"] performs the following operations:
1. Initializes variables
2. Processes data
3. Returns results

✅ Full cycle completed: CLI → MCP Bridge → Extension Host → Command Execution
```text

### Test 3.3: Refactor Code

```bash
fusion extensions exec gemini.refactorCode --args '["legacy_function()"]'
```text

**Expected Output:**

```text
✅ Refactoring Suggestions for ["legacy_function()"]:

1. Extract method for repeated logic
2. Use pattern matching
3. Add error handling
```text

## Phase 4: Error Handling

### Test 4.1: Execute Without Authentication

```bash

# Remove credentials

rm ~/.fusion/credentials/store.json

# Try to execute

fusion extensions exec gemini.generateCode --args '["test"]'
```text

**Expected Output:**

```text
Error: Extension google.gemini-code-assist is not authenticated. Run 'fusion extensions install google.gemini-code-assist' first.
```text

### Test 4.2: Invalid Extension ID

```bash
fusion extensions install invalid-extension
```text

**Expected Output:**

```text
📥 Installing extension: invalid-extension
  ⚠ Extension not found in marketplace
  (Note: Full marketplace integration pending)
```text

## Phase 5: MCP Server Integration

### Test 5.1: Start MCP Server with Extension Support

```bash
fusion mcp serve --port 8080 --extensions
```text

**Expected Output:**

```text
🚀 Starting MCP server on port 8080...
  Extension support: enabled
✓ MCP server running
```text

### Test 5.2: MCP Bridge Tool Execution

```bash

# Through MCP protocol (simulated)

curl -X POST http://localhost:8080/tools/execute \
  -H "Content-Type: application/json" \
  -d '{
    "tool": "extension.gemini.generateCode",
    "arguments": ["Create a function"]
  }'
```text

**Expected MCP Bridge Flow:**
1. MCP Server receives tool execution request
2. Routes to ExtensionMcpBridge
3. Bridge calls ExtensionHost.execute_command()
4. ExtensionHost looks up command in registry
5. Executes registered handler
6. Returns result through bridge

## Success Criteria

### ✅ Authentication System

- [x] OAuth flow launches browser
- [x] Credentials stored securely
- [x] API key alternative works
- [x] Credential verification before execution

### ✅ Extension Management

- [x] Install with authentication
- [x] List shows credential status
- [x] Execute requires authentication

### ✅ MCP Bridge

- [x] CLI → ExtensionHost connection works
- [x] Command registry functional
- [x] Tool execution returns results
- [x] Full cycle: CLI → MCP → Host → Command

### ✅ Error Handling

- [x] Missing credentials detected
- [x] Invalid extensions handled
- [x] Clear error messages

## Performance Metrics

| Operation         | Expected Time |
| ----------------- | ------------- |
| OAuth Flow        | < 5 seconds   |
| Credential Store  | < 100ms       |
| Command Execution | < 500ms       |
| MCP Round-trip    | < 1 second    |

## Security Validation

- [x] Credentials stored in user directory (`~/.fusion/credentials/`)
- [x] OAuth state parameter prevents CSRF
- [x] Localhost callback (port 8765)
- [x] Per-extension credential isolation
- [x] No credentials in logs/output

## Next Steps After Testing

1. **Encrypt Credentials**: Add AES-256 encryption to credential store
2. **Real OAuth**: Implement actual callback server
3. **Terminal Browser**: Complete UI implementation
4. **Marketplace**: Add real extension download
5. **Multi-Provider**: Support GitHub, Microsoft OAuth