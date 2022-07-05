# node-webp

```ts
const nodeImagePng = require('node-image-webp');

// 找到文件中所有的图片
const imageList = nodeImagePng.findAllUrl(fileName);

// 从 url 获取图片并转转化为 webp
nodeImagePng.webImageToWebp(url, distPath);

// 本地图片并转转化为 webp
nodeImagePng.imageToWebp(srcPath, distPath);

// 将 distPath 上传到cdn
const { url, ...reset } = await zipAndUpload(item.distPath, false);
```
