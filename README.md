# IBoardBot API

Recreated IBoardBot API in Rust.

![board with globe, linux tux and atom drawn](./images/img0.jpg)

# Building

Clone the repository and run `cargo build` or `cargo run` to run it.

The API will start on port 8080.

# Reconfiguring the board

You'll need to reconfigure (and reflash) the board so it points to the new API instead of the old JJRobots API.

Simply open the Configuration.h file and find the following lines

```c
#define SERVER_HOST "0.0.0.0"
#define SERVER_URL "http://0.0.0.0:8080/_/board/main"
```

Note that instead of using 0.0.0.0 you'll need to put the actual ip of the server / computer where you are hosting the API.

If you want to connect multiple boards, the "main" is the board name, so you can change that to represent different boards, eg:

```c
#define SERVER_URL "http://0.0.0.0:8080/_/board/other-board-1"
#define SERVER_URL "http://0.0.0.0:8080/_/board/my-board"
// etc
```

The first time that the board connects to the API, the API will create a configuration file for that board under `./boards/main.yaml` (or whatever you called your board). There you can set the board dimensions. You can then check if those dimensions are alright by queue-ing the Calibrate job, which will draw a rectangle based on the specified board bounds.

# API Docs

You can find the API docs at the `/docs` endpoint in the API.
