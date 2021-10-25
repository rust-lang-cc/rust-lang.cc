
# tailwind

使用 tailwind 的过程，做个笔记，方便以后查阅。  

## TOC
[use tailwind directly](#use-tailwind-directly)
[use tailwind with framework](#use-tailwind-with-framework)  
[my usage of tailwind](#my-usage)
[resources](#resources)


tailwind 可以直接使用，也可以跟其它 前端框架 一起使用。

### use-tailwind-directly  
[back to TOC](#TOC)  

直接使用 tailwind 方式如下  
(一般直接使用 tailwind 时可结合 alpine.js 或直接跟后端集成)  

https://flaviocopes.com/tailwind-setup/  
https://tailwindcss.com/docs/installation

mkdir tw-start && cd tw-start
npm install -D tailwindcss@latest postcss@latest autoprefixer@latest

npx tailwindcss init -p
生成并修改 tailwind.config.js，这里 -p 同时生成了 postcss.config.js  
module.exports = {
  purge: [],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [],
}


修改 tailwind.config.js 的 purge 部分  
  purge: [
    './src/**/*.html',
    './src/**/*.js',
   ],

修改 postcss.config.js (没有的话手动新建) (用上面的 -p 生成出来的本来就这样，不用修改)  
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  }
}

新建 src/rustlangcc.css  
@tailwind base;
@tailwind components;
@tailwind utilities;

新建 src/index.html
```html
<!doctype html>
<html>
<head>
  <!-- ... -->
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link href="./tailwind_purged.css" rel="stylesheet">
  <title>tailwind demo</title>
</head>
<body class="bg-gray-400">
  <!-- ... -->
  <div class="container mx-auto sm:w-screen md:max-w-2xl bg-gray-300 p-4">
    <h1>Welcome to tailwindcss demo</h1>
  </div>
</body>
</html>
```

pwsh  
$env:NODE_ENV="production"; npx tailwindcss -i ./src/rustlangcc.css -o ./src/tailwind_purged.css  
exit  
每次运行完就退出 pwsh，要运行就进入 pwsh。因为有时候要安装别的依赖不能在 pwsh 里面，如下面的 daisyui.  
上面这样生成出来的样式，h1 h2 h3 以及链接等等什么样式都没有，页面所有文字全一个样，甚至连链接都看不出来。

用浏览器打开 index.html 看看现在的效果。


npm install @tailwindcss/typography@latest --save-dev
// 上面 这个命令不能在 pwsh 里面运行，要直接在 cmd 下面运行安装  

修改 tailwind.config.js,  在 plugins 增加： 
    require('@tailwindcss/typography'),


使用 daisyui  
npm i daisyui

修改 tailwind.config.js,  在 plugins 增加：  
  plugins: [
    require('daisyui'),
  ],

这里 require('daisyui'), 要放在 @tailwindcss/typography 的下一行。(来自 daisyui 文档 )  


修改 src/index.html  
在外层的 div 使用 prose, 并增加一个daisyui 的 button, 如下设置：  
<div class="container mx-auto sm:w-screen md:max-w-2xl bg-gray-300 p-4 prose">
  <h1>Welcome to tailwindcss demo</h1>
  <button class="btn btn-primary">DaisyUI Button</button>
</div>

再次生成 tailwind_purged.css  
pwsh  
// $env:NODE_ENV="production"; npx tailwindcss -i ./src/rustlangcc.css -o ./src/tailwind_purged.css  
$env:NODE_ENV="production"; npx tailwindcss -i ./rustlangcc.css -o ./tailwind_purged.css  
exit  

用浏览器打开 index.html 看看现在的效果。




#### 其它  

安装(获得)一个最原始的 tailwind.css 文件  
npx tailwindcss@latest -o tailwind_full.css  
(tailwindcss-cli 已失效过期，以后直接使用 tailwindcss 作为 cli)
('The `tailwindcss-cli` package has been deprecated, use `tailwindcss` instead.')

[关于 tailwind.config.js 的设置](https://www.tailwindcss.cn/docs/configuration)  
[tailwind.config.js 设置 英文网站](https://tailwindcss.com/docs/configuration)  


npm init -y
生成并修改 package.json,在 scripts 里面增加  
// "build:css": "postcss src/tailwind.css -o static/dist/tailwind.css"
"build:css": "npx tailwindcss src/tailwind.css -o static/dist/tailwind.css"

如果要压缩生成的 css， 就要安装 cssnano 并运行  
NODE_ENV=production npx tailwindcss -i ./src/tailwind.css -o ./dist/tailwind.css --minify

加 watch 参数，当有文件修改时自己编译生成新的 css 目标文件。  
npx tailwindcss -o tailwind.css --watch

#### 使用 purgecss (作用类似上面直接使用 npx tailwindcss)

使用 purgecss 来按 html 使用过的 css 简化
// purgecss usage:  
npm i -g purgecss  
purgecss -css tailwind.css --content index.html --output ./ tailwind_purged.min.css  
[purgecss usage](https://www.purgecss.cn/CLI.html)  


### use-tailwind-with-framework  
[back to TOC](#TOC)  

tailwind 比较常用的用法是跟前端框架一起使用  
如跟 react, vue, nextjs 等等  
我在不用 react 的时候就直接用上面的直接使用方式。  


### my-usage  
[back to TOC](#TOC)  

我使用 tailwind 的方式

Tailwind 默认内置了 5 个宽度断点  
https://www.tailwindcss.cn/docs/responsive-design  
- sm 640px
- md 768px
- lg 1024px
- xl 1280px
- 2xl 1536px
可以在 tailwind.config.js 文件中完全自定义断点
[tailwind 完整的默认设置](https://unpkg.com/browse/tailwindcss@2.2.7/stubs/defaultConfig.stub.js)  
spacing, 4 是 1rem, 最大 96 是 24rem  
[max-width](https://tailwindcss.com/docs/max-width)  

默认模板  
直接参看上面的。  



#### resources  
[back to TOC](#TOC)  

有用的资源  
[play tailwind](https://play.tailwindcss.com/)  
[play tailwind with daisy ui](https://play.tailwindcss.com/R74XalS4na)  
[daisyui: tailwind components](daisyui.com)  
[daisyui github](https://github.com/saadeghi/daisyui)  
[tailblocks](https://tailblocks.cc/)  
[destack](https://github.com/LiveDuo/destack)  
[destack demo](https://destack-page.vercel.app/)  
Page builder based on Next.js 🅧, Tailwind CSS leaves & Grapes.js grapes. Zero-config deployment to Vercel  
[merakiui based on tailwind](https://merakiui.com/)  
[tailwind components](https://tailwindcomponents.com/)  
[tailwind cheatsheet](https://tailwindcomponents.com/cheatsheet/)  

[AlpineJS meets TailwindCSS](https://codepen.io/collection/XqVbyQ)  
[AlpineJS toolbox and examples](https://www.alpinetoolbox.com/examples/)  
[Tailwind + alpine examples](https://www.tailwindawesome.com/)  

[tailwindcss.cn](https://www.tailwindcss.cn/)  
[tailwind traders.com](https://tailwindtraders.com/)  

