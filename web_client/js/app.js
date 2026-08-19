(function () {
    let currentDimension = 'overworld';
    let tileFormat = 'webp';
    let maxZoom = 4;
    let currentZoom = 2;

    fetch('/api/status')
        .then(res => res.json())
        .then(data => {
            if (data.format) {
                tileFormat = data.format;
            }
            if (data.max_zoom !== undefined) {
                maxZoom = data.max_zoom;
            }
            initMap();
        })
        .catch(() => {
            initMap();
        });

    let map = null;
    let tileLayer = null;

    function initMap() {
        if (map) {
            map.remove();
        }

        const AmberCRS = L.Util.extend({}, L.CRS.Simple, {
            transformation: new L.Transformation(1 / 256, 0, 1 / 256, 0),
        });

        map = L.map('map', {
            crs: AmberCRS,
            minZoom: 0,
            maxZoom: maxZoom + 2,
            zoomControl: false,
            attributionControl: false,
        });

        L.control.zoom({ position: 'topright' }).addTo(map);

        const AmberTileLayer = L.TileLayer.extend({
            getTileUrl: function (coords) {
                const z = Math.min(coords.z, maxZoom);
                const x = coords.x;
                const y = coords.y;
                return `/tiles/${currentDimension}/${z}/${x}/${y}.${tileFormat}`;
            }
        });

        tileLayer = new AmberTileLayer('', {
            tileSize: 256,
            noWrap: true,
            maxNativeZoom: maxZoom,
            errorTileUrl: 'data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256"><rect width="256" height="256" fill="rgba(15,23,42,0.6)"/></svg>',
        }).addTo(map);

        map.setView([0, 0], Math.min(2, maxZoom));

        const hudBlock = document.getElementById('hud-block-coords');
        const hudChunk = document.getElementById('hud-chunk-coords');
        const hudZoom = document.getElementById('hud-zoom-level');

        function updateCoords(latlng) {
            const realBlockX = Math.round(latlng.lng * 256);
            const realBlockZ = Math.round(latlng.lat * 256);

            const chunkX = Math.floor(realBlockX / 16);
            const chunkZ = Math.floor(realBlockZ / 16);

            hudBlock.innerHTML = `X: ${realBlockX.toLocaleString()} &nbsp;|&nbsp; Z: ${realBlockZ.toLocaleString()}`;
            hudChunk.textContent = `${chunkX.toLocaleString()}, ${chunkZ.toLocaleString()}`;
            hudZoom.textContent = `Z: ${map.getZoom()}`;
        }

        map.on('mousemove', function (e) {
            updateCoords(e.latlng);
        });

        map.on('zoomend', function () {
            hudZoom.textContent = `Z: ${map.getZoom()}`;
        });

        document.getElementById('btn-recenter').addEventListener('click', function () {
            map.flyTo([0, 0], Math.min(2, maxZoom), { duration: 0.6 });
        });

        document.querySelectorAll('.dim-btn').forEach(btn => {
            btn.addEventListener('click', function () {
                document.querySelectorAll('.dim-btn').forEach(b => b.classList.remove('active'));
                this.classList.add('active');
                currentDimension = this.dataset.dim;
                tileLayer.redraw();
            });
        });
    }
})();
