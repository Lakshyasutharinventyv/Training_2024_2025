
## Overview
This project demonstrates an authentication UI featuring Login and Register tabs. The tabs are implemented using semantic HTML elements (`<details>` and `<summary>`), ensuring functionality without relying on JavaScript, radio buttons, checkboxes, or anchor tags.

## Features
Login and Register Tabs: Switch between Login and Register forms using native HTML `<details>` and `<summary>` tags.
Responsive Design: Ensures usability across different devices and screen sizes.
CSS Styling:
Modern, gradient-based UI for the right panel.
Clean and minimalist design for input fields and buttons.
No JavaScript: Tab switching is entirely handled by HTML and CSS.

## Implementation Details
Authentication Tabs
The tabs are created using semantic `<details>` and `<summary>` elements. When a tab is opened, the other tab is automatically collapsed due to the native behavior of `<details>`.

## HTML Structure:
``` HTML
  <div class="tabs">
    <details id="login-details" open>
      <summary>Login</summary>
      <div class="tab-content">
        <!-- Login Form -->
      </div>
    </details>

    <details id="register-details">
      <summary>Register</summary>
      <div class="tab-content">
        <!-- Register Form -->
      </div>
    </details>
  </div>
```

### CSS Highlights:
Styling ensures only one tab is expanded at a time.
Improved form visuals with transitions and responsive adjustments.

``` css
  details {
    width: 48%;
  }

  details[open] {
    width: 30vw;
    position: absolute;
  }

  .tab-content {
    padding: 20px;
    border-radius: 0 0 4px 4px;
    display: none;
  }

  details[open] .tab-content {
    display: block;
  }
```

### Advantages of This Approach
No JavaScript Dependency: Simplifies the project and improves accessibility.
Native HTML Behavior: Uses `<details>` and `<summary>` to handle collapsible tabs, ensuring broader compatibility.
Ease of Maintenance: Simple, clean, and semantic code structure.


### Future Enhancements
Add animations for smoother transitions.
Improve accessibility with ARIA attributes.
Integrate backend functionality for form submissions.

### License
This project is licensed under the MIT License.