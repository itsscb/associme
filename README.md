# associme

## ⚠️ PROJECT STATUS: EARLY CONCEPT / NOT FUNCTIONAL ⚠️

`associme` is an ambitious project aimed at providing a batteries-included webservice for associations, such as soccer clubs, to manage their members, events, and more. Currently, this is a proof-of-concept and is far from being usable. Built with `Rust` and `Angular`, it aims to provide a comprehensive solution for association management.

# Demo

Currently the latest release is hosted at [associme-xlnd.shuttle.app/](https://associme-xlnd.shuttle.app/).

The userlogin is `admin` with the same as password.

**Please refrain from exploiting the login credentials provided for the demo.
<br>This is a temporary setup intended for demonstration purposes only.**

# Features

- [-] Member management
- [ ] Event organization
- [ ] Financial tracking
- [ ] Communication tools
- [ ] Customizable dashboard

# Prerequisites

## Nix users (flakes)

Run `nix develop` inside the cloned repository.

## All others

Before you begin, ensure you have the following installed:

- Rust (_latest stable version_)
- Node.js (_v22.11 or later_)
- npm (_v10.9 or later_)
- Angular CLI (_v16.1.8 or later_)
- Docker (_for local development_)
- Shuttle CLI

To install the Shuttle CLI, run:

```bash
cargo install shuttle
```

# Usage

## Clone the repository:

```bash
git clone https://github.com/itsscb/associme.git && cd associme
```

## Run locally:

```bash
shuttle run
```

Open your browser and navigate to http://localhost:8000 to access the `associme` application.

# Development

## Backend

The backend is built with Rust and uses the Shuttle.rs framework. To start the development server:

```bash
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
cd frontend/
npm run br
cd ..
shuttle deploy
```

# Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

# License

This project is licensed under the GNU General Public License v3.0 License.
