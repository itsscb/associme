# associme

## ⚠️ PROJECT STATUS: EARLY CONCEPT / NOT FUNCTIONAL ⚠️

`associme` is an ambitious project aimed at providing a batteries-included webservice for associations, such as soccer clubs, to manage their members, events, and more. Currently, this is a proof-of-concept and is far from being usable. Built with `Rust` and `Angular`, it aims to provide a comprehensive solution for association management.

# Features

- [-] Member management
- [ ] Event organization
- [ ] Financial tracking
- [ ] Communication tools
- [ ] Customizable dashboard

# Prerequisites

Before you begin, ensure you have the following installed:

- Rust (latest stable version)
- Node.js (v22.11 or later)
- npm (v10.9 or later)
- Angular CLI (v16.1.8 or later)
- Shuttle CLI

To install the Shuttle CLI, run:

```bash
curl -sSf https://www.shuttle.rs/install.sh | bash
```

# Usage

## Clone the repository:

```bash
git clone https://github.com/itsscb/associme.git
cd associme
```

## Run locally:

```bash
cd backend
shuttle run
```

Open your browser and navigate to http://localhost:8000 to access the `associme` application.

# Development

## Backend

The backend is built with Rust and uses the Shuttle.rs framework. To start the development server:

```bash
cd backend
shuttle run
```

## Frontend

The frontend is an Angular application. To start the development server:

```bash
cd frontend
ng serve
```

## Deployment

`associme` can be easily deployed using Shuttle.rs. To deploy your application:

### Ensure you have a Shuttle account and are logged in:

```bash
shuttle login
```

### Link your project directory to a Shuttle project:

```bash
shuttle project link
```

### Deploy your application:

```bash
shuttle deploy
```

# Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

# License

This project is licensed under the MIT License.
