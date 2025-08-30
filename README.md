<!-- PROJECT LOGO -->
<br />
<div align="center">
 <a href="https://git.prisma.moe/aichan/image_gallery">
 </a>

 <h1 align="center">Image Gallery</h1>
 <p align="center"> <img 
    src="https://visitcounter.aichan.ovh/counter/image_gallery/svg?label=Project%20Visits" height=20
    alt="Visit Counter" /> </p>

 <p align="center">
 A responsive image gallery built with Rust Yew and WebAssembly. Features dynamic image loading, smooth animations, and Docker deployment for showcasing photo collections.
 <br />
 <br />
 <a href="https://yew.rs/docs/getting-started/introduction">Yew Documentation</a>
 ·
 <a href="https://git.prisma.moe/aichan/image_gallery/issues">Report Bug</a>
 ·
 <a href="https://git.prisma.moe/aichan/image_gallery/issues">Request Feature</a>
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
 <a href="#architecture">Architecture</a>
 <ul>
 <li><a href="#gallery-frontend">Gallery Frontend</a></li>
 <li><a href="#image-discovery">Image Discovery</a></li>
 </ul>
 </li>
 <li><a href="#features">Features</a></li>
 <li><a href="#deployment">Deployment</a></li>
   <ul>
      <li><a href="#using-docker">Using Docker</a></li>
   </ul>
   <ul>
      <li><a href="#using-docker-compose">Using Docker Compose</a></li>
   </ul>
 <li><a href="#development">Development</a></li>
   <ul>
      <li><a href="#prerequisites">Prerequisites</a></li>
      <li><a href="#local-development">Local Development</a></li>
   </ul>
 <li><a href="#contributing">Contributing</a></li>
 <li><a href="#license">License</a></li>
 <li><a href="#contact">Contact</a></li>
 </ol>
</details>

## About The Project

Image Gallery is a modern, responsive web application for displaying photo collections. Built with Rust and WebAssembly, it provides fast loading times and smooth user interactions. The application automatically discovers images in a directory and generates a dynamic gallery interface with lazy loading and responsive design.

Perfect for photographers, artists, or anyone wanting to showcase their image collections with a clean, professional interface.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white) [![Yew](https://img.shields.io/badge/Yew-2E8B57?style=for-the-badge&logo=rust&logoColor=white)](#) [![Bash](https://img.shields.io/badge/Bash-4EAA25?style=for-the-badge&logo=gnubash&logoColor=fff)](#) ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white) ![Nginx](https://img.shields.io/badge/nginx-%23009639.svg?style=for-the-badge&logo=nginx&logoColor=white)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Architecture

The Image Gallery uses a simple but effective architecture that separates image discovery from the frontend display.

### Gallery Frontend
**Technology:** Rust Yew (compiled to WebAssembly)

- Responsive grid layout that adapts to different screen sizes
- Lazy loading for optimal performance with large image collections
- Smooth animations and transitions
- JSON-based image metadata loading
- Modern CSS Grid and Flexbox styling

### Image Discovery
**Technology:** Bash script automation

- Automatic scanning of image directories
- JSON metadata generation with image paths and metadata
- File system monitoring for dynamic updates
- Support for common image formats (JPG, PNG, GIF, WebP)

<p align="right">(<a href="#architecture">back to top</a>)</p>

## Features

- 📸 **Dynamic Image Loading** - Automatically discovers and displays images from directories
- 🎨 **Responsive Design** - Adapts seamlessly to desktop, tablet, and mobile devices
- ⚡ **WebAssembly Performance** - Fast, native-speed execution in the browser
- 🖼️ **Lazy Loading** - Images load as needed for optimal performance
- 📱 **Touch-Friendly** - Optimized for mobile interactions and gestures
- 🔄 **Auto-Discovery** - Automatically updates when new images are added
- 🐳 **Container Ready** - Easy deployment with Docker and Docker Compose

<p align="right">(<a href="#features">back to top</a>)</p>

## Deployment

You have multiple options to deploy the Image Gallery:

### Using Docker

Run the pre-built container:
```bash
docker run --rm -it \
  -p 80:80/tcp \
  -e GALLERY_TITLE="Image Gallery" \
  -e GALLERY_HEADING="Welcome!" \
  -e GALLERY_DESCRIPTION="This is the gallery description that you can customize with GALLERY_DESCRIPTION env variable and even supports <a href=\"https://git.prisma.moe/aichan/image_gallery\">links like this</a>." \
  -e VISIT_COUNTER_URL="https://visitcounter.aichan.ovh/counter/gallery_example/svg?label=Visits&background_label=00000000&background_counter=00000000&shadow_opacity=0&grad_stop1_color=00000000&grad_stop1_opacity=0&grad_stop1_opacity=0&grad_stop2_opacity=0" \
  -v /path/to/your/images:/usr/share/nginx/html/static/images \
  git.prisma.moe/aichan/image_gallery:latest
```

Replace 'gallery_example' with your domain or unique identifier

### Using Docker Compose

Create a `docker-compose.yml` file:
```yaml
services:
  image_gallery:
    container_name: image_gallery
    image: git.prisma.moe/aichan/image_gallery:latest
    ports:
      - <YOUR_PORT>:80
    environment:
      - GALLERY_TITLE=Image Gallery
      - GALLERY_HEADING=Welcome!
      - GALLERY_DESCRIPTION=This is the gallery description that you can customize with GALLERY_DESCRIPTION env variable and even supports <a href="https://git.prisma.moe/aichan/image_gallery">links like this</a>.
      - FOOTER_LINK_TEXT=Repository
      - FOOTER_LINK_URL=https://git.prisma.moe/aichan/image_gallery
      - VISIT_COUNTER_URL=https://visitcounter.aichan.ovh/counter/gallery/svg?label=Visits&background_label=00000000&background_counter=00000000&shadow_opacity=0&grad_stop1_color=00000000&grad_stop1_opacity=0&grad_stop1_opacity=0&grad_stop2_opacity=0  # Replace 'gallery_example' with your domain or unique identifier
    volumes:
      - <PATH_TO_YOUR_IMAGES>:/usr/share/nginx/html/static/images
    restart: unless-stopped
```

Then run:
```bash
docker-compose up -d
```

**Access Point:**
- Gallery will be served at `http://localhost:<YOUR_PORT>/`

<p align="right">(<a href="#deployment">back to top</a>)</p>

## Development

### Prerequisites

- Rust installed (includes Cargo)
- WebAssembly target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- Trunk for building and serving:
  ```bash
  cargo install --locked trunk
  ```
- (Optional) Docker for containerized deployment

### Local Development

1. Clone the repository:
   ```bash
   git clone https://git.prisma.moe/aichan/image_gallery.git
   cd image_gallery
   ```

2. Add your images to the `static/images/` directory

3. Generate the images JSON:
   ```bash
   ./generate_images_json.sh
   ```

4. Run the development server:
   ```bash
   trunk serve --address 0.0.0.0
   ```

5. Open your browser to `http://localhost:8080`

**Development Notes:**
- The `generate_images_json.sh` script scans the `static/images/` directory and creates `static/images.json`
- Any changes to the Rust code require a rebuild
- CSS changes in `styles.css` are applied immediately
- Add new images to `static/images/` and regenerate the JSON for them to appear

<p align="right">(<a href="#development">back to top</a>)</p>

## Contributing

Contributions are welcome! Please fork the repository, make your changes, and open a pull request.

1. Fork the Project on [Forgejo](https://git.prisma.moe/aichan/image_gallery)
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

Project Link: [https://git.prisma.moe/aichan/image_gallery](https://git.prisma.moe/aichan/image_gallery)

<p align="right">(<a href="#contact">back to top</a>)</p>

## Acknowledgments

Thanks to these amazing projects and technologies!

- [Rust Yew](https://yew.rs/) - A modern Rust framework for creating multi-threaded front-end web apps with WebAssembly
- [WebAssembly](https://webassembly.org/) - A binary instruction format for a stack-based virtual machine

<p align="right">(<a href="#readme-top">back to top</a>)</p>