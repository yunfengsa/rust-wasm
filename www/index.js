import * as wasm from "../pkg";

// 点击导入按钮,使files触发点击事件,然后完成读取文件的操作
$("#fileImport").click(function () {
    $("#files").click();
})
window.fileImport = function() {
    //获取读取我文件的File对象
    var selectedFile = document.getElementById('files').files[0];
    var reader = new FileReader();//这是核心,读取操作就是由它完成.
    reader.readAsArrayBuffer(selectedFile);//读取文件的内容,也可以读取文件的URL
    reader.onload = function () {
        var uint8Array = new Uint8Array(this.result);
        wasm.grayscale(uint8Array);
    }
}