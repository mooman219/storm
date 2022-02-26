var asset_payloads;

export function push_asset(index, paths) {
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
        (asset_payloads ||= []).push([index, array]);
    });
}

export function pull_assets() {
    let temp = asset_payloads;
    asset_payloads = [];
    return temp || [];
}