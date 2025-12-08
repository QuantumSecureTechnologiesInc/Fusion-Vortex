// src/web/mod.fu - WebAssembly and DOM Interaction Library

use fusion::async::Task;
use fusion::collections::List;
use fusion::runtime::{Result, Error};
use fusion::ui::Element; // Reusing the UI element concept for the DOM

// --- Low-Level JavaScript Bridge (Compiler Intrinsics) ---

// Compiler intrinsic to call a global JavaScript function.
// The fusionc compiler knows to treat functions starting with 'js_' as FFI imports.
fn js_get_element_by_id(id: String) -> ElementPointer;
fn js_set_inner_html(ptr: ElementPointer, html: String);
fn js_add_event_listener(ptr: ElementPointer, event: String, callback: fn(Event) -> Task<()>);
fn js_fetch_url(url: String) -> Task<String>;

struct ElementPointer; // Opaque pointer to a DOM element
struct Event; // Opaque event object

// --- High-Level DOM Manipulation ---

/// Represents a managed DOM element.
struct DomElement:
    ptr: ElementPointer
    
    static fn select_by_id(id: String) -> Result<DomElement>:
        let ptr = js_get_element_by_id(id);
        if ptr.is_null():
            return Err(Error::new("DomElementNotFound", format!("ID '{}' not found.", id)))
        return Ok(DomElement { ptr: ptr })
        
    fn set_content(self, content: String):
        js_set_inner_html(self.ptr, content)

    /// Registers an asynchronous event handler.
    fn on_click(self, handler: fn(Event) -> Task<()>):
        js_add_event_listener(self.ptr, "click", handler)

// --- Web Networking ---

/// Wrapper for asynchronous network requests.
async fn fetch(url: String) -> Result<String>:
    // Delegates to the JS host's native fetch API for efficiency.
    return js_fetch_url(url).await.map_err(|e| Error::new("NetworkError", e))

// --- Example Usage (Browser-side) ---

fn update_dom_content(id: String, text: String) -> Task<()> {
    match DomElement::select_by_id(id) {
        case Ok(element):
            element.set_content(text);
        case Err(e):
            println!("Error updating DOM: {}", e.message);
    }
    return Task::completed()
}

// Full application example using the declarative UI model (fusion::ui)
fn main_wasm_app() -> Task<()> {
    // 1. Initial DOM update
    update_dom_content("status", "Application initializing...").await;

    // 2. Fetch data asynchronously
    let fetch_task = async {
        match fetch("https://api.fusion-quantum.com/data").await {
            case Ok(data):
                update_dom_content("data_output", format!("Data received: {}", data)).await;
            case Err(e):
                update_dom_content("data_output", format!("Fetch failed: {}", e.message)).await;
        }
    };
    
    // 3. Define event handler that performs a secure operation
    let button = DomElement::select_by_id("sign_button").expect("Button not found");
    button.on_click(|_event| async {
        // Example: Use the hybrid crypto module compiled to WASM
        use fusion::crypto::{hybrid_sign, HybridKeypair};
        
        let keys = HybridKeypair::generate().expect("Key gen failed");
        let message = "Secure transaction hash".as_bytes();
        
        let signature = hybrid_sign(
            message, 
            &keys.classical_sig.private_key, 
            &keys.pqc_sig.private_key
        ).expect("Signing failed");
        
        update_dom_content("status", format!("Hybrid Signed (PQC+Classical). Sig length: {}", signature.classical_sig.len + signature.pqc_sig.len)).await;
    });

    // Run the fetch task concurrently
    fetch_task
}