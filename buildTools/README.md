# Vue3-Simple-Icons Build Tools

## Requirements
- [Deno](https://deno.com/)


This tool will loop through all the the Icons Exported by Simple-Icons and make a Vue Component Based on the Icon

## Command Parameters
- `--target` Sets the output folder for the components
- `--simple-icons` The Source Folder for the Icons.  Defaults to `node_modules/simple-icons`
- `--info-json` Where you want to Write the Info Json


## Info Json
This file is used by Simple-Icons to show the current version of Simple Icons used in this project. This also allows you to look up the Component Name vs The slug name. 
