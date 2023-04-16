var fs_callback;

export function fs_load_files(index, paths) {
    let promises = paths.map(function (path) {
        return fetch(path).then(function (response) {
            if (response.status < 200 || response.status >= 300) {
                return response.status;
            } else {
                return response.arrayBuffer().then(function (buffer) {
                    return new Uint8Array(buffer);
                }).catch(function (reason) {
                    return 500;
                });
            }
        }).catch(function (reason) {
            return 500;
        });
    });
    Promise.all(promises).then(function (array) {
        fs_callback(index, array);
    });
}

export function fs_init_callback(callback) {
    fs_callback = callback;
}
