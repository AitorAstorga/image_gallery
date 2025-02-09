[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
 <a href="https://github.com/AitorAstorga/image_gallery">
 </a>

 <h1 align="center">Image Gallery</h1>
 <p align="center"> <img 
    src="https://visitcounter.aichan.ovh/counter/YOUR_PAGE_NAME/svg?label=Example%20Visits" height=20
    alt="Visit Counter" /> </p>

 <p align="center">
 Simple HTML / CSS / Webasm (<a href="https://yew.rs/">Rust Yew</a>) image gallery made for <a href="https://michi.blue/">my cat</a> and deployed with Docker.
 <br />
 <br />
 <a href="https://yew.rs/docs/getting-started/introduction">Yew Documentation</a>
 ·
 <a href="https://github.com/AitorAstorga/image_gallery/issues">Report Bug</a>
 ·
 <a href="https://github.com/AitorAstorga/image_gallery/issues">Request Feature</a>
 </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
 <summary>Table of Contents</summary>
 <ol>
 <li><a href="#about-the-project">About The Project</a></li>
   <ul>
      <li><a href="#built-with">Built With</a></li>
   </ul>
 </li>
 <li>
 <a href="#project-structure">Project Structure</a>
 <ul>
 <li><a href="#modules">Modules</a></li>
 </ul>
 </li>
 <li><a href="#getting-started">Getting Started</a></li>
    <ul>
      <li><a href="#prerequisites">Prerequisites</a></li>
   </ul>
   <ul>
      <li><a href="#installation-and-custom-ui">Installation and Custom UI</a></li>
   </ul>
 <li><a href="#deploying-your-custom-app">Deploying Your Custom App</a></li>
   <ul>
      <li><a href="#option-a-push-to-ghcrio-using-github-actions">Option A: Push to ghcr.io Using GitHub Actions</a></li>
   </ul>
   <ul>
      <li><a href="#option-B-use-the-dockerfile-directly">Option B: Use the Dockerfile Directly</a></li>
   </ul>
 <li><a href="#contributing">Contributing</a></li>
 <li><a href="#license">License</a></li>
 <li><a href="#contact">Contact</a></li>
 </ol>
</details>

## About The Project

This project is an image gallery application that dynamically generates a JSON list of images from a specified directory. It uses Nginx to serve the content and monitors the images directory for any changes, updating the JSON file accordingly. The application is containerised using Docker and integrated with GitHub Actions for continuous integration and deployment.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) [![Bash](https://img.shields.io/badge/Bash-4EAA25?style=for-the-badge&logo=gnubash&logoColor=fff)](#) ˙ [![GitHub Actions](https://img.shields.io/badge/GitHub_Actions-2088FF?style=for-the-badge&logo=github-actions&logoColor=white)](#) ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white) ![Nginx](https://img.shields.io/badge/nginx-%23009639.svg?style=for-the-badge&logo=nginx&logoColor=white) ˙ [![Visual Studio Code](https://custom-icon-badges.demolab.com/badge/Visual%20Studio%20Code-0078d7.svg?style=for-the-badge&logo=vsc&logoColor=white)](#)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Project Structure

The project is organised as follows:

```
image_gallery/
├── Cargo.toml
├── generate_images_json.sh # Script to serve as a local "API" discovering the images
├── static/
│ ├── images/
│ ├── resources/
│ └── images.json # Needed for the page to know what images to load
└── src/
  ├── main.rs
  ├── models.rs # Data structures
  ├── router.rs # Define the URL routes
  ├── components/
  │ ├── gallery.rs
  │ └── mod.rs
  ├── pages/
  │ ├── home.rs
  │ └── mod.rs
```

### Modules

- `main.rs`: Entrypoint of the application.
- `models.rs`: Contains data structures for counters and SVG options.
- `router.rs`: Controls the directing of the URL (just / our case)
- `components/gallery.rs`: Fetches the images and generates the grid with them.
- `pages/home.rs`: Contains the main logic and HTML code.

<p align="right">(<a href="#project-structure">back to top</a>)</p>

## Getting Started

Follow these instructions to set up a local instance of the Image Gallery.

> [!NOTE]
> You have to build this locally or fork this repository to be able to modify the HTML to your liking.

### Prerequisites

- Rust installed (includes Cargo)
- WebAssembly target
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
- Trunk
   ```bash
   cargo install --locked trunk
   ```
- (Optional) Docker if you plan to deploy in a container

### Installation and Custom UI

1. Clone the repository:
   ```bash
   git clone https://github.com/AitorAstorga/image_gallery.git
   cd image_gallery
   ```

2. Modify the HTML
   - Edit the File:
     Open `/src/pages/home.rs`. It contains the HTML (and Yew component code) for the home page.

   - Make Your Changes:
     Update the HTML, styles, or layout as desired. Remember that because this is a Rust Yew app, any UI changes will require a rebuild for them to take effect.

3. Run the application locally:
   ```bash
   trunk serve --address 0.0.0.0
   ```

By default, Yew runs on `localhost:8080`.

<p align="right">(<a href="#testing-the-api">back to top</a>)</p>

## Deploying Your Custom App

You have two options to deploy your changes:

### Option A: Push to ghcr.io Using GitHub Actions

You can use GitHub Actions to automatically build the project and push a Docker image to GitHub Container Registry (ghcr.io).

1. Ensure the Workflow Exists:
   Check that your fork contains the GitHub Actions workflow (found in .github/workflows/). It should be configured to build your project and push the image.

2. Configure Repository Secrets:
   In your forked repository, set up the following secrets (via **Settings > Secrets and variables > Actions**):
   - `GHCR_TOKEN`: Your GitHub Container Registry Personal Access Token.
   - (Any other secrets required by your workflow.)

3. Commit and Push Your Changes:
   ```bash
   git add .
   git commit -m "Customize home page HTML"
   git push origin main
   ```

   The GitHub Actions workflow will trigger automatically, build your project, and push the Docker image to:
   ```bash
   ghcr.io/<your-username>/aichan-image-gallery:latest
   ```

4. Update docker-compose (if needed):
   If you use Docker Compose, update the image reference in your `docker-compose.yml` to point to your custom image.

   A sample. Keep in mind this will use my image, you should make your own.
   ```yaml
   services:
   web:
      container_name: image-gallery
      image: ghcr.io/aitorastorga/aichan-image-gallery:latest
      ports:
         - "YOUR_PORT:80"
      volumes:
         - /PATH_TO_YOUR_IMAGES:/usr/share/nginx/html/static/images
      restart: unless-stopped
   ```

### Option B: Use the Dockerfile Directly

If you prefer local container builds, use the provided `Dockerfile`:

1. Build the Docker Image:
   ```bash
   docker build -t your-image-name .
   ```

2. Run the Container:
   ```bash
   docker run -d -p YOUR_PORT:80 your-image-name
   ```

3. Update docker-compose.yml (Optional):
   If you deploy with Docker Compose, modify the image field in docker-compose.yml to use your newly built image.

> [!NOTE]
> Reminder: Every time you modify the UI or other front-end code (such as /src/pages/home.rs), be sure to rebuild your project (or Docker image) so that your changes are included in your deployment.

<p align="right">(<a href="#getting-started">back to top</a>)</p>

## Contributing

Contributions are welcome! Please fork the repository, make your changes, and open a pull request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#contributing">back to top</a>)</p>

## License

Distributed under the European Union Public License v1.2. See `LICENSE` for more information.

<p align="right">(<a href="#license">back to top</a>)</p>

## Contact

Aitor Astorga Saez de Vicuña - a.astorga.sdv@protonmail.com

Project Link: [https://github.com/AitorAstorga/image_gallery](https://github.com/AitorAstorga/image_gallery)

<p align="right">(<a href="#contact">back to top</a>)</p>

## Acknowledgments

Thanks to these nice projects!

- [Rust Yew](https://yew.rs/) A framework for creating reliable and efficient web applications.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/AitorAstorga/image_gallery.svg?style=for-the-badge
[contributors-url]: https://github.com/AitorAstorga/image_gallery/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/AitorAstorga/image_gallery.svg?style=for-the-badge
[forks-url]: https://github.com/AitorAstorga/image_gallery/network/members
[stars-shield]: https://img.shields.io/github/stars/AitorAstorga/image_gallery.svg?style=for-the-badge
[stars-url]: https://github.com/AitorAstorga/image_gallery/stargazers
[issues-shield]: https://img.shields.io/github/issues/AitorAstorga/image_gallery.svg?style=for-the-badge
[issues-url]: https://github.com/AitorAstorga/image_gallery/issues
[license-shield]: https://img.shields.io/github/license/AitorAstorga/image_gallery.svg?style=for-the-badge
[license-url]: https://github.com/AitorAstorga/image_gallery/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/aitor-astorga-saez-de-vicuña
