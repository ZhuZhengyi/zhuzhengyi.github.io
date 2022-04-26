L2Dwidget.init({
    "model": {
        jsonPath: "https://unpkg.com/live2d-widget-model-shizuku@1.0.5/assets/shizuku.model.json",
        hHeadPos: 0.5,
        vHeadPos: 0.618,
        scale: 1
    },
    "display": {
        "position": "right",
        //"superSample": 2,
        "width": 100,
        "height": 200,
        "hOffset": 10,
        "vOffset": 10
    },
    "mobile": {
        "motion": true,
        "show": false,
        "scale": 0.5
    },
    "react": {
        "opacityDefault": 0.7,
        "opacityOnHover": 0.2
    }
});

