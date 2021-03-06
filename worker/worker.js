addEventListener('fetch', (event) => {
    if (event.request.method.toUpperCase() !== 'POST') {
        return event.respondWith(new Response(`POST a .png to this address and I'll resize it for you`));
    }
    event.respondWith(handlePostRequest(event));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handlePostRequest(event) {
    const { resize } = wasm_bindgen;
    await wasm_bindgen(wasm);
    let bytes = new Uint8Array(await event.request.arrayBuffer());
    const resized = resize(bytes);
    return new Response(resized, { status: 200 });
}
