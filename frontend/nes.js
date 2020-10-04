var ctx;
var imageData;
var rustWasm;
var emulatorPtr;
var romBytes;

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
  rustWasm = await wasmInit("../target/wasm32-unknown-unknown/release/NEruSt.wasm");

    const wasmMemory = new Uint8Array(rustWasm.instance.exports.memory.buffer);
    var romBytesLen = romBytes.byteLength;
    bytePtr = rustWasm.instance.exports.allocate_bytes(romBytesLen);

    for (var i = 0; i < romBytesLen; i++) {
        wasmMemory[bytePtr + i] = romBytes[i];
    }

    console.log(bytePtr);
    console.log(romBytes);
    console.log(romBytesLen);
    emulatorPtr = rustWasm.instance.exports.create_emulator(bytePtr, romBytesLen);
    rustWasm.instance.exports.free_bytes(bytePtr, romBytesLen);

    window.requestAnimationFrame(runFrame);
};

const runFrame = () => {
    rustWasm.instance.exports.run_frame(emulatorPtr);
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
}
