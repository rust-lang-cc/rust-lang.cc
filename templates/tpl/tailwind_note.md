
# tailwind

使用 tailwind 的过程，做个笔记，方便以后查阅。  

## TOC
[use tailwind directly](#use-tailwind-directly)
    [tailwind.config.js](#tailwindconfigjs)
[use tailwind with framework](#use-tailwind-with-framework)  
[my usage of tailwind](#my-usage)
[resources](#resources)

tailwind 可以直接使用，也可以跟其它 前端框架 一起使用。

### use-tailwind-directly

直接使用 tailwind 方式如下  
(一般直接使用 tailwind 时结合 alpine.js 或直接跟后端集成)  

安装(获得)一个最原始的 tailwind.css 文件  
npx tailwindcss@latest build -o tailwind.css  
(tailwindcss-cli 已失效过期，以后直接使用 tailwindcss 作为 cli)
('The `tailwindcss-cli` package has been deprecated, use `tailwindcss` instead.')

#### tailwind.config.js  
生成原始的 tailwind.config.js  
npx tailwindcss init  
这将会在您的工程根目录创建一个最小的 tailwind.config.js 文件。  
```
// tailwind.config.js
module.exports = {
  purge: [],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {},
  plugins: [],
}
```

[关于 tailwind.config.js 的设置](https://www.tailwindcss.cn/docs/configuration)  


然后运行如下  
```shell
npm i -g tailwindcss
pwsh
$env:NODE_ENV="production"; tailwindcss build -o tailwind_purged.min.css
```

#### 使用 purgecss (作用类似上面直接使用 tailwind)

使用 purgecss 来按 html 使用过的 css 简化
// purgecss usage:  
npm i -g purgecss  
purgecss -css tailwind.css --content index.html --output ./ tailwind_purged.min.css  
[purgecss usage](https://www.purgecss.cn/CLI.html)  


### use-tailwind-with-framework

tailwind 比较常用的用法是跟前端框架一起使用  
如跟 react, vue, nextjs 等等  
我在不用 react 的时候就直接用上面的直接使用方式。  


### my-usage

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
```html
<!doctype html>
<html>
<head>
  <!-- ... -->
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link href="./tailwind.css" rel="stylesheet">
</head>
<body>
  <!-- ... -->
  <div class="container mx-auto sm:w-screen md:max-w-2xl">

  </div>
</body>
</html>
```

#### resources

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

