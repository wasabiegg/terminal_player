<br />
<p align="center">

  <h3 align="center">terminal_player</h3>

  <p align="center">
    Terminal gif/video player
    <br />
    <a href="https://github.com/wasabiegg/terminal_player/issues">Report Bug</a>
    ·
    <a href="https://github.com/wasabiegg/terminal_player/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <!-- <li><a href="#acknowledgements">Acknowledgements</a></li> -->
  </ol>
</details>



<!-- about the project -->
## about the project
[![Product Name Screen Shot][project-demo]](https://github.com/wasabiegg/terminal_player)



<!-- GETTING STARTED -->
## Getting Started


### Prerequisites

> Only Linux

* rustc and cargo [download](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/wasabiegg/terminal_player
   ```
2. Install dependency
   ```sh
   cd terminal_player/
   cargo build --release
   ```
   > executable file is ./target/release/terminal_player



<!-- USAGE EXAMPLES -->
## Usage

1. auto width and height
  ```sh
  terminal_player path_to_gif_file --fps 25
  ```
2. set width or height or both
  ```sh
  terminal_player path_to_gif_file --fps 25 -w 80 
  terminal_player path_to_gif_file --fps 25 -h 80
  terminal_player path_to_gif_file --fps 25 -w 80 -h 80
  ```
3. Have fun!



<!-- ROADMAP -->
## Roadmap
- [ ] BUG FIXING

See the [open issues](https://github.com/wasabiegg/terminal_player) for a list of proposed features (and known issues).



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/wasabiegg/terminal_player](https://github.com/wasabiegg/terminal_player)


<!-- ACKNOWLEDGEMENTS -->
<!-- ## Acknowledgements
* [GitHub Emoji Cheat Sheet](https://www.webpagefx.com/tools/emoji-cheat-sheet)
* [Img Shields](https://shields.io)
* [Choose an Open Source License](https://choosealicense.com)
* [GitHub Pages](https://pages.github.com)
* [Animate.css](https://daneden.github.io/animate.css)
* [Loaders.css](https://connoratherton.com/loaders)
* [Slick Carousel](https://kenwheeler.github.io/slick)
* [Smooth Scroll](https://github.com/cferdinandi/smooth-scroll)
* [Sticky Kit](http://leafo.net/sticky-kit)
* [JVectorMap](http://jvectormap.com)
* [Font Awesome](https://fontawesome.com) -->





<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
<!-- [contributors-url]: https://github.com/othneildrew/Best-README-Template/graphs/contributors
[forks-url]: https://github.com/othneildrew/Best-README-Template/network/members
[stars-url]: https://github.com/othneildrew/Best-README-Template/stargazers
[issues-url]: https://github.com/othneildrew/Best-README-Template/issues
[license-url]: https://github.com/othneildrew/Best-README-Template/blob/master/LICENSE.txt -->
[project-demo]: images/demo.gif
