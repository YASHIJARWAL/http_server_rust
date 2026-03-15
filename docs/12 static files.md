# Static File Serving in the Rust HTTP Server

## Introduction

Static file serving is a fundamental capability of web servers.
It allows the server to return **files stored on disk** directly to the client.

Common static assets include:

* HTML files
* CSS stylesheets
* JavaScript files
* Images
* JSON files
* Fonts

In this project, static files are served from the **public directory**.

---

# What is a Static File?

A static file is a file whose content **does not change dynamically** when requested.

Examples:

```
index.html
style.css
script.js
logo.png
```

The server simply reads the file from disk and returns it to the browser.

---

# Why Static File Serving Is Important

Most modern web applications rely on static assets.

Example web page loading process:

```
Browser
  │
Request index.html
  │
Server returns HTML
  │
HTML references CSS and JS
  │
Browser requests style.css
Browser requests script.js
  │
Server returns assets
```

Without static file support, the server cannot host a full web interface.

---

# Static File Flow

```
Browser Request
       │
GET /style.css
       │
Server maps path
       │
public/style.css
       │
File read from disk
       │
Response created
       │
File returned to browser
```

---

# Path Mapping

Incoming requests are mapped to the filesystem.

Example mappings:

| Request     | File              |
| ----------- | ----------------- |
| /index.html | public/index.html |
| /style.css  | public/style.css  |
| /script.js  | public/script.js  |

---

# MIME Types

When serving files, the server must specify the correct **Content-Type** header.

Example HTTP response:

```
HTTP/1.1 200 OK
Content-Type: text/css
Content-Length: 125

body {
  background-color: violet;
}
```

This tells the browser how to interpret the file.

---

# Static File Function

The server checks whether a file exists and returns it if found.

Key steps:

```
1. Construct file path
2. Check file existence
3. Read file contents
4. Determine MIME type
5. Build response
6. Send response
```

---

# Benefits of Static File Serving

Static file serving allows the server to:

* Host websites
* Serve frontend assets
* Deliver CSS and JavaScript
* Provide images and media
* Support simple web applications

---

# Limitations in Current Implementation

The current implementation uses:

```
fs::read_to_string()
```

This works for **text-based files** such as:

* HTML
* CSS
* JavaScript

However, binary files like images should use:

```
fs::read()
```

This improvement will be addressed in a later phase.

---

# Summary

Static file serving is a core feature of any web server.

With this capability, the Rust HTTP server can now:

* Deliver frontend assets
* Support complete web pages
* Host simple static websites

This marks an important milestone toward building a **full-featured backend server architecture**.
