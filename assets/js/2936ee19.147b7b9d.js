"use strict";(self.webpackChunkh3_website=self.webpackChunkh3_website||[]).push([[4183],{3905:function(e,t,r){r.d(t,{Zo:function(){return u},kt:function(){return p}});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function o(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var c=n.createContext({}),l=function(e){var t=n.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):o(o({},t),e)),r},u=function(e){var t=l(e.components);return n.createElement(c.Provider,{value:t},e.children)},h={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,i=e.originalType,c=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),d=l(r),p=a,f=d["".concat(c,".").concat(p)]||d[p]||h[p]||i;return r?n.createElement(f,o(o({ref:t},u),{},{components:r})):n.createElement(f,o({ref:t},u))}));function p(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=r.length,o=new Array(i);o[0]=d;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s.mdxType="string"==typeof e?e:a,o[1]=s;for(var l=2;l<i;l++)o[l]=r[l];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},5989:function(e,t,r){r.r(t),r.d(t,{frontMatter:function(){return s},contentTitle:function(){return c},metadata:function(){return l},toc:function(){return u},default:function(){return d}});var n=r(3117),a=r(102),i=(r(7294),r(3905)),o=["components"],s={id:"geohash",title:"Geohash",sidebar_label:"Geohash",slug:"/comparisons/geohash"},c=void 0,l={unversionedId:"comparisons/geohash",id:"comparisons/geohash",isDocsHomePage:!1,title:"Geohash",description:"Geohash is a system for encoding locations using a string of characters, creating a hierarchical, square grid system (a quadtree).",source:"@site/docs/comparisons/geohash.md",sourceDirName:"comparisons",slug:"/comparisons/geohash",permalink:"/docs/next/comparisons/geohash",editUrl:"https://github.com/uber/h3/edit/master/website/docs/comparisons/geohash.md",tags:[],version:"current",frontMatter:{id:"geohash",title:"Geohash",sidebar_label:"Geohash",slug:"/comparisons/geohash"},sidebar:"someSidebar",previous:{title:"S2",permalink:"/docs/next/comparisons/s2"},next:{title:"Hexbin",permalink:"/docs/next/comparisons/hexbin"}},u=[{value:"Area distortion",id:"area-distortion",children:[],level:2},{value:"Identifiers",id:"identifiers",children:[],level:2},{value:"Geohash vs H3 Comparison",id:"geohash-vs-h3-comparison",children:[],level:2}],h={toc:u};function d(e){var t=e.components,r=(0,a.Z)(e,o);return(0,i.kt)("wrapper",(0,n.Z)({},h,r,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,(0,i.kt)("a",{parentName:"p",href:"https://en.wikipedia.org/wiki/Geohash"},"Geohash")," is a system for encoding locations using a string of characters, creating a hierarchical, square grid system (a quadtree)."),(0,i.kt)("h2",{id:"area-distortion"},"Area distortion"),(0,i.kt)("p",null,"Because Geohash encodes latitude and longitudes pairs, it is subject to significant differences in area at different latitudes. A degree of longitude near a pole represents a significantly smaller distance than a degree of longitude near the equator."),(0,i.kt)("h2",{id:"identifiers"},"Identifiers"),(0,i.kt)("p",null,"Geohash uses strings for its cell indexes. Because they are strings, they can encode arbitrarily precise cells."),(0,i.kt)("p",null,"H3 cell indexes are designed to be 64 bit integers, which can be rendered and transmitted as strings if needed. The integer representation can be used when high performance is needed, as integer operations are usually more performant than string operations. Because indexes are fixed size, H3 has a maximum resolution it can encode."),(0,i.kt)("h2",{id:"geohash-vs-h3-comparison"},"Geohash vs H3 Comparison"),(0,i.kt)("iframe",{width:"100%",height:"500px",src:"https://studio.unfolded.ai/public/009a4f1e-2b74-4c0f-a156-95051c6583f3/embed",frameborder:"0",allowfullscreen:!0}),(0,i.kt)("p",null,"Geohash on the left, H3 on the right. Data: ",(0,i.kt)("a",{parentName:"p",href:"https://data.sfgov.org/City-Infrastructure/Street-Tree-List/tkzw-k3nq"},"San Francisco Street Tree List")))}d.isMDXComponent=!0}}]);