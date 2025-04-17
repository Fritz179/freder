# TODO

- [ ] Math
    - [ ] Complete `Vec2`

- [ ] Rendering
    - [ ] Add `View`
        - [ ] Define scaling options
            - [ ] With buffer or direct?
            - [ ] Define option vs options trait for shapes
    - [ ] Add `Colors`

- [ ] Window


- [ ] Refactor
    - [ ] Transform should take a owned transform and have helper functions


Canvas vs View 
    View can be like save_state
    But then canvas has more memory overhead?
        - Transform + Clipping options approx = 20 bytes => negligible

Draw vs Render
    Draw takes an object and options
    Render only the object
    Who owns and modifies the object? 

    canvas.render(entity) should not modify entity, entity itself calls draw an the canvas

    canvas.draw(shape, options) own the shape and options, can therefore modify them

    When are lines scaled? 
        - set_pixel vs sampler?
        - canvas asks sempler what pixel each coordinate is => images 
        - commands tells canvas which pixels are drawn => resolution?
        - nearest neighbor scaling? 0.5x vs 2x scaling?
        - NN -> can get a 2x2, or a buffer, or other settings?
        - temporary buffer and then scaled?


Marker?
    Markers are drawn at the end, over other things, for debugging and now pixelated event if buffer canvas is used
    Z layers?
    When should they be cleared? copy them into the imageDrawer, but then they must be cleared before reuse


Position corner or center of pixel?
    Scaled line marker should be in the middle, dots/cross also
    It remains in the corner for rects? 
    What if it is scaled x4? Player position is float?


Position vs Size
    Position easy to transform, size is special?
    

Where does the Event get Transform2D?
    - get_mouse_pos(&self, transform: &Transform2D) -> Vec2
    - get_mouse_pos_raw(&self) -> Vec2

    What if nested transforms?

// Simple Canvas, only for images
// Canvas, transfrom + markers?