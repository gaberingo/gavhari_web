## Using DaisyUI with Yew

*Folder Structure*
```bash
frontend
├── Cargo.lock
├── Cargo.toml
├── dist
├── docs
│   └── using_daisyui_standalone.md
├── index.html
├── src
│   ├── components
│   │   ├── header.rs
│   │   └── mod.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── pages
│   │   ├── base.rs
│   │   └── mod.rs
│   └── router.rs
├── static
│   └── src
│       ├── css
│       │   └── tailwind.css
│       ├── js
│       │   ├── daisyui.js
│       │   └── daisyui-theme.js
│       └── tailwindcss
└── Trunk.toml
```

1. Download the Tailwind CSS executable
```bash
# Run the appropriate command for your OS
# Linux
curl -sLo tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64

# For other OS, check https://github.com/tailwindlabs/tailwindcss/releases/latest
```

Make the file executable (for Linux and macOS):

```bash
chmod +x tailwindcss
```

2. Download the daisyUI standalone JS files
```bash
curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.js
curl -sLO https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.js
```

Then, place the Tailwind CSS and DaisyUI files in the `frontend` folder as shown above.

3. Configure Tailwind CSS and daisyUI

*tailwind.css*
```css
@import "tailwindcss" source(none);
@source "./src/**/*.{rs,html}"; /* Scan Rust (.rs) and HTML files in the src folder */
@plugin "./daisyui.js"; /* Add daisyUI standalone plugin */

/* Optional theme customization */
@plugin "./daisyui-theme.js" {
    /* Example: change primary color */
    --color-primary: #00ff00;
}
```

4. Add Tailwind configuration

*tailwind.config.js*
```js
// tailwind.config.js
module.exports = {
    content: [
        './src/**/*.rs', 
        './index.html',
    ],
    theme: {
        extend: {
            colors: {
                // Customize primary color (example)
                primary: "#00ff00",
            },
        },
    },
    plugins: [
        // Add any Tailwind CSS plugins you want to use here
    ]
}
```

5. Build Tailwind CSS with DaisyUI using Trunk

*Trunk.toml*
```toml
[build]
target = "index.html"
dist = "dist"

[[hooks]]
stage = "build"
command = "sh"
command_arguments = ["-c", "./tailwindcss -i ./tailwind.css -o $TRUNK_STAGING_DIR/tailwind.css"]
```

6. Use Tailwind CSS and DaisyUI in your index file

*index.html*
```html
<!DOCTYPE html>
<html lang="en">
<head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        
        <!-- Tailwind CSS -->
        <link rel="stylesheet" href="/tailwind.css"/>
        <base data-trunk-public-url/>

        <title>Gavhari</title>
</head>
<body>
        
</body>
</html>
```