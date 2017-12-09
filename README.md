**This repo is an experimental, project, with the intent to learn Rust by building a semi-useful
tool**

Ever wonder what npm scripts are in your package.json file? Well wonder no more! With this handy little script, you can log out just the scritps to the terminal!

This works well with having a bash alias that sends the current working directory as the cli argument. Such as
```shell
alias scripts='pwd | ~/which_npm_scripts/target/release/which_npm_scripts'
```

Then its as simple as
```shell
~/some_contrived_app_name $ scripts
{
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test --env=jsdom",
    "eject": "react-scripts eject"
}
```
