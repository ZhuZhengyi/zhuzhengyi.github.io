L2Dwidget.init({
    model: {
        jsonPath: "https://unpkg.com/live2d-widget-model-shizuku@1.0.5/assets/shizuku.model.json",
        hHeadPos: 0.5,
        vHeadPos: 0.618,
        scale: 0.7
    },
    display: {
        position: 'left',   // 显示位置：左或右
        superSample: 9,     // 超采样等级，相当于清晰度，数值越高越清晰
        width: 80,         // canvas的宽度
        height: 160,        // canvas的高度
        hOffset: 10,         // canvas水平偏移，正方向为右
        vOffset: 10,         // canvas垂直偏移
    },
    mobile: {
        show: false,         // 是否在移动设备上显示
        motion: true,       // 移动设备是否开启重力感应
        scale: 0.5,           // 移动设备上的缩放
    },
    react: {
        opacityDefault: 0.7,  // 默认透明度
        opacityOnHover: 0.2,  // 鼠标移上透明度
    },
});

