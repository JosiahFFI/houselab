# Houselab (egui) Roadmap

## Basic editing

- [x] Sidebar navigation
- [x] Section editing
    - [x] Name, description, inline
    - [x] Add, remove, move
    - [x] Comments
        - [x] Apply/summary comments
        - [x] Edit comment lists/entries
        - [x] Add comments
        - [x] Edit comments

- [x] Inspection details editing
    - [x] Name, description
    - [x] Date
    - [x] Inspectors, client, seller (from premade list)

- [x] Multiple pages framework w/ buttons in header
- [x] Person editor (inspectors, clients, realtors)
- [o] Android support
    - [x] Opens on android
    - [o] Relatively ergonomic controls on android

## Images

- [ ] Separate images tab per section
- [ ] Load image from file
- [ ] Camera integration

- [ ] Move/duplicate/remove images
- [ ] Send image to other section

- [ ] Annotate images
    - [ ] Add basic annotations (line, circle, rect)

- [ ] Separate images tab for whole inspection
    - [ ] Show all images
    - [ ] Location selector per image
    - [ ] Individual/batch image compression

## Application styling

- [ ] Light mode theme
- [ ] Proper scaling on android
- [ ] Proper wrapping/scrolling on all platforms

## Inspection management

- [ ] Save/load inspections from file
- [ ] Add support for saving inspections with images

## Export

- [ ] Make PDF export
    - [ ] Basic, ugly export
    - [ ] Built-in theming
    - [ ] Embedded (downscaled) images
    - [ ] Images with overlays

## Network sync

- [ ] Delta system
    - [ ] Build delta encoding for sending incremental changes
    - [ ] Save deltas on each edit

- [ ] LAN connection
    - [ ] Find local peers automatically
    - [ ] Send whole inspection to peer on join
    - [ ] Send deltas between peers on edits
    - [ ] Handle transient network errors gracefully
    - [ ] Handle disconnecting and reconnecting later gracefully

- [ ] WAN connection
    - [ ] Setup remote server for interchange
    - [ ] Setup password-protected "rooms"
    - [ ] Handle network transiency and eventual consistency

## Template editing

- [ ] Allow editing basic colors/fonts/logos
- [ ] Find some templating language to allow editing the final PDF's appearance

## Platform support

- [ ] Thoroughly test on Linux, Windows, and Android
- [ ] Ensure app remains usable on smaller devices (like phones)
