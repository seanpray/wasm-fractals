import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const realInput = document.getElementById('real');
        const imaginaryInput = document.getElementById('imaginary');
        const cut_offInput = document.getElementById('cut_off');
        const renderBtn = document.getElementById('render');
        const selectionInput = document.getElementById('selection');
        console.log(selectionInput)

        renderBtn.addEventListener('click', () => {
            let time = document.getElementById('time');
            const start = Date.now();
            const real = parseFloat(realInput.value) || 0;
            const imaginary = parseFloat(imaginaryInput.value) || 0;
            const cut_off = parseInt(cut_offInput.value) || 500;
            const selection = document.getElementById('selection');
            const picked = selection.options[selection.selectedIndex].value;
            console.log(selection.options[selection.selectedIndex].value)
            //select.options[select.selectedIndex].value
            wasm.draw(ctx, 750, 750, picked, real, imaginary, cut_off);
            time.innerHTML = (Date.now() - start);
        });
        let time = document.getElementById('time');
        const start = Date.now();
        wasm.draw(ctx,  750, 750, "julia", -0.15, 0.65, 500);
        time.innerHTML = (Date.now() - start);
    })
    .catch(console.error);
