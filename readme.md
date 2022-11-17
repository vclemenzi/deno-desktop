# Deno Desktop

Result of many testing with deno and rust for creating desktop app with web technologies


## Instructions

```bash
deno task start
```

`/` 
- `loader.ts` - load all modules and build the app
- `mod.ts` - Render the app
- `app.config` - App configuration
- `*other` - Other files

`app/` *App directory*
- `App.tsx` *Principal app file*
- `style.css` *App style*

`rust/` *Rust directory*
- `*` *Rust files for build*

`.out/` *Output directory (generated after build)*
- `*` *Output files*