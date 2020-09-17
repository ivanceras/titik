
## TODO controls
- [x] box (base, extensible)
- [x] scrollbox box
- [X] button
- [x] checkbox
- [X] column ( vbox )
- [x] row ( hbox )
- [x] container
- [x] image
    - [ ] event listeners
        - [ ] click event
        - [ ] mouse move event
- [ ] progress_bar
- [x] radio
      - [ ] event listener
        - [ ] on input
- [ ] scrollbars
- [ ] slider
    - [ ] event listener
       - [ ] on input
- [x] text
- [X] text_input (textbox)
       - [ ] event listener
       - [x] on input
- [ ] text_area
    - [ ] scrollbar dynamic to the relative content size
    - [ ] scrollbar can be dragged
    - [ ] scrollbar moves relative to the scroll location
    - [ ] event listener
         - [X] on input
         - [ ] on scroll event
- [X] groupbox
    - [ ] ganged radio button in the same group
- [ ] combobox, dropdown box

- [ ] process the key modifiers in events
- [ ] Fix a bug in inner buffer of flexbox, 1-line off
- [ ] Charts
   - [ ] bar charts
   - [ ] line chart
       - [ ] vertical
       - [ ] horizontal
   - [ ] scatter point charts
- [ ] All widget will have to be wrapped with `Rc<Mutex<_>>` in order
 for it to have multiple owner that can modify the UI

## Document
- [ ] document each function in the widget interface
