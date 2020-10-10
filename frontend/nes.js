var ctx;
var imageData;
var rustWasm;
var emulatorPtr;
var romBytes;
var player1Controller = 0;
var player2Controller = 0;

const buttons = {
    RIGHT:  0x01,
    LEFT:   0x02,
    DOWN:   0x04,
    UP:     0x08,
    START:  0x10,
    SELECT: 0x20,
    B:      0x40,
    A:      0x80,
}

async function start() {
    console.log("start");
    const canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');
    
    ctx.fillStyle = 'green';
    ctx.fillRect(10, 10, 150, 100);

    imageData = ctx.createImageData(256, 240);
    await loadWasm();
}

function draw_to_screen(ptr) {
    console.log("draw to screen " + ptr);
    var imageData = ctx.getImageData(0, 0, 256, 240);
    var data = imageData.data;

    const wasmMemory = new Uint8Array(rustWasm.instance.exports.memory.buffer);

    for (var y = 0; y < 240; y++) {
        for (var x = 0; x < 256; x++) {
            const wasmIdx = ((y * 256) + x) * 3;
            const screenIdx = ((y * 256) + x) * 4;
            // RGB
            for (var i = 0; i < 3; i++) {
                data[screenIdx + i] = wasmMemory[ptr + wasmIdx + i];
            }
            data[screenIdx + 3] = 255;
        }
    }
    ctx.putImageData(imageData, 0, 0);
}

const wasmInit = async (wasmModuleUrl, importObject) => {
    console.log("wasmInit");

  if (!importObject) {
    importObject = {
      env: {
          draw_screen: (ptr) => draw_to_screen(ptr),
      }
    };
  }

    const wasmArrayBuffer = await fetch(wasmModuleUrl).then(response =>
        response.arrayBuffer()
    );
    return WebAssembly.instantiate(wasmArrayBuffer, importObject);
};

const loadWasm = async () => {
    console.log("loadWasm");
  //rustWasm = await wasmInit("../target/wasm32-unknown-unknown/release/NEruSt.wasm");
  rustWasm = await wasmInit("../target/wasm32-unknown-unknown/debug/NEruSt.wasm");

    var romBytesLen = romBytes.byteLength;
    bytePtr = rustWasm.instance.exports.allocate_bytes(romBytesLen);
    var wasmMemory = new Uint8Array(rustWasm.instance.exports.memory.buffer);

    for (var i = 0; i < romBytesLen; i++) {
        wasmMemory[bytePtr + i] = romBytes[i];
    }

    emulatorPtr = rustWasm.instance.exports.create_emulator(bytePtr, romBytesLen);
    rustWasm.instance.exports.free_bytes(bytePtr, romBytesLen);

    window.requestAnimationFrame(runFrame);
};

const runFrame = () => {
    // TODO: handle endianness, ensure this is little endian on all platforms.
    const input = (player2Controller << 8) | player1Controller;
    rustWasm.instance.exports.run_frame(emulatorPtr, input);
    window.requestAnimationFrame(runFrame);
};

// TOOD: figure out wtf is going on with drag and drop...
// https://stackoverflow.com/questions/8006715/drag-drop-files-into-standard-html-file-input
// https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API/File_drag_and_drop
class dropHandler {
};

dropHandler.ondragover = dropHandler.ondragenter = function(evt) {
    evt.preventDefault();
};

// handles dropping a file to load a ROM
dropHandler.ondrop = function(ev) {
    console.log('File(s) dropped');
    
    fileInput.files = evt.dataTransfer.files;
    
    // If you want to use some of the dropped files
    const dT = new DataTransfer();
    dT.items.add(evt.dataTransfer.files[0]);
    dT.items.add(evt.dataTransfer.files[3]);
    fileInput.files = dT.files;

    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();
}

function handleKeyDown(e) {
    if (e.keyCode == 37) {
        // left arrow pressed
        player1Controller |= buttons.LEFT;
    } else if ( e.keyCode == 38) {
        // up arrow pressed
        player1Controller |= buttons.UP;
    } else if ( e.keyCode == 39) {
        // right arrow pressed
        player1Controller |= buttons.RIGHT;
    } else if ( e.keyCode == 40) {
        // down arrow pressed
        player1Controller |= buttons.DOWN;
    } else if ( e.keyCode == 65) {
        // A on qwerty keycode pressed
        player1Controller |= buttons.A;
    } else if ( e.keyCode == 79) {
        // S on qwerty keycode pressed
        player1Controller |= buttons.B;
    } 
}

function handleKeyUp(e) {
    if (e.keyCode == 37) {
        // left arrow released
        player1Controller &= ~buttons.LEFT;
    } else if ( e.keyCode == 38) {
        // up arrow released
        player1Controller &= ~buttons.UP;
    } else if ( e.keyCode == 39) {
        // right arrow released
        player1Controller &= ~buttons.RIGHT;
    } else if ( e.keyCode == 40) {
        // down arrow released
        player1Controller &= ~buttons.DOWN;
    } else if ( e.keyCode == 65) {
        // A on qwerty keycode released
        player1Controller &= ~buttons.A;
    } else if ( e.keyCode == 79) {
        // S on qwerty keycode released
        player1Controller &= ~buttons.B;
    } 
}


function setUpHandlers() {
    // file handling
    document.getElementById('fileInput').addEventListener('change', function() {
        var reader = new FileReader();
        reader.onload = function() {
            romBytes = new Uint8Array(this.result);
            console.log(romBytes);
        }
        reader.readAsArrayBuffer(this.files[0]);
    }, false);

    // TODO: add gamepad support
    // https://developer.mozilla.org/en-US/docs/Web/API/Gamepad_API/Using_the_Gamepad_API
    window.addEventListener("gamepadconnected", function(e) {
        console.log("Gamepad connected at index %d: %s. %d buttons, %d axes.",
                    e.gamepad.index, e.gamepad.id,
                    e.gamepad.buttons.length, e.gamepad.axes.length);
    });

    // user input support
    //var canvas = document.getElementById('canvas')
    // couldn't get it working on canvas directly... not sure why
    window.addEventListener( 'keydown', handleKeyDown, true );
    window.addEventListener( 'keyup', handleKeyUp, true );
}
