"use strict";(self.webpackChunkh3_website=self.webpackChunkh3_website||[]).push([[5609],{3905:function(e,n,t){t.d(n,{Zo:function(){return c},kt:function(){return m}});var a=t(7294);function l(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function r(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);n&&(a=a.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,a)}return t}function o(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?r(Object(t),!0).forEach((function(n){l(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):r(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function u(e,n){if(null==e)return{};var t,a,l=function(e,n){if(null==e)return{};var t,a,l={},r=Object.keys(e);for(a=0;a<r.length;a++)t=r[a],n.indexOf(t)>=0||(l[t]=e[t]);return l}(e,n);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(a=0;a<r.length;a++)t=r[a],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(l[t]=e[t])}return l}var i=a.createContext({}),s=function(e){var n=a.useContext(i),t=n;return e&&(t="function"==typeof e?e(n):o(o({},n),e)),t},c=function(e){var n=s(e.components);return a.createElement(i.Provider,{value:n},e.children)},p={inlineCode:"code",wrapper:function(e){var n=e.children;return a.createElement(a.Fragment,{},n)}},d=a.forwardRef((function(e,n){var t=e.components,l=e.mdxType,r=e.originalType,i=e.parentName,c=u(e,["components","mdxType","originalType","parentName"]),d=s(t),m=l,b=d["".concat(i,".").concat(m)]||d[m]||p[m]||r;return t?a.createElement(b,o(o({ref:n},c),{},{components:t})):a.createElement(b,o({ref:n},c))}));function m(e,n){var t=arguments,l=n&&n.mdxType;if("string"==typeof e||l){var r=t.length,o=new Array(r);o[0]=d;var u={};for(var i in n)hasOwnProperty.call(n,i)&&(u[i]=n[i]);u.originalType=e,u.mdxType="string"==typeof e?e:l,o[1]=u;for(var s=2;s<r;s++)o[s]=t[s];return a.createElement.apply(null,o)}return a.createElement.apply(null,t)}d.displayName="MDXCreateElement"},8215:function(e,n,t){var a=t(7294);n.Z=function(e){var n=e.children,t=e.hidden,l=e.className;return a.createElement("div",{role:"tabpanel",hidden:t,className:l},n)}},6396:function(e,n,t){t.d(n,{Z:function(){return d}});var a=t(3117),l=t(7294),r=t(2389),o=t(9443);var u=function(){var e=(0,l.useContext)(o.Z);if(null==e)throw new Error('"useUserPreferencesContext" is used outside of "Layout" component.');return e},i=t(9521),s=t(6010),c="tabItem_1uMI";function p(e){var n,t,a,r=e.lazy,o=e.block,p=e.defaultValue,d=e.values,m=e.groupId,b=e.className,f=l.Children.map(e.children,(function(e){if((0,l.isValidElement)(e)&&void 0!==e.props.value)return e;throw new Error("Docusaurus error: Bad <Tabs> child <"+("string"==typeof e.type?e.type:e.type.name)+'>: all children of the <Tabs> component should be <TabItem>, and every <TabItem> should have a unique "value" prop.')})),v=null!=d?d:f.map((function(e){var n=e.props;return{value:n.value,label:n.label}})),g=(0,i.lx)(v,(function(e,n){return e.value===n.value}));if(g.length>0)throw new Error('Docusaurus error: Duplicate values "'+g.map((function(e){return e.value})).join(", ")+'" found in <Tabs>. Every value needs to be unique.');var h=null===p?p:null!=(n=null!=p?p:null==(t=f.find((function(e){return e.props.default})))?void 0:t.props.value)?n:null==(a=f[0])?void 0:a.props.value;if(null!==h&&!v.some((function(e){return e.value===h})))throw new Error('Docusaurus error: The <Tabs> has a defaultValue "'+h+'" but none of its children has the corresponding value. Available values are: '+v.map((function(e){return e.value})).join(", ")+". If you intend to show no default tab, use defaultValue={null} instead.");var k=u(),y=k.tabGroupChoices,w=k.setTabGroupChoices,N=(0,l.useState)(h),x=N[0],T=N[1],I=[],O=(0,i.o5)().blockElementScrollPositionUntilNextRender;if(null!=m){var E=y[m];null!=E&&E!==x&&v.some((function(e){return e.value===E}))&&T(E)}var j=function(e){var n=e.currentTarget,t=I.indexOf(n),a=v[t].value;a!==x&&(O(n),T(a),null!=m&&w(m,a))},P=function(e){var n,t=null;switch(e.key){case"ArrowRight":var a=I.indexOf(e.currentTarget)+1;t=I[a]||I[0];break;case"ArrowLeft":var l=I.indexOf(e.currentTarget)-1;t=I[l]||I[I.length-1]}null==(n=t)||n.focus()};return l.createElement("div",{className:"tabs-container"},l.createElement("ul",{role:"tablist","aria-orientation":"horizontal",className:(0,s.Z)("tabs",{"tabs--block":o},b)},v.map((function(e){var n=e.value,t=e.label;return l.createElement("li",{role:"tab",tabIndex:x===n?0:-1,"aria-selected":x===n,className:(0,s.Z)("tabs__item",c,{"tabs__item--active":x===n}),key:n,ref:function(e){return I.push(e)},onKeyDown:P,onFocus:j,onClick:j},null!=t?t:n)}))),r?(0,l.cloneElement)(f.filter((function(e){return e.props.value===x}))[0],{className:"margin-vert--md"}):l.createElement("div",{className:"margin-vert--md"},f.map((function(e,n){return(0,l.cloneElement)(e,{key:n,hidden:e.props.value!==x})}))))}function d(e){var n=(0,r.Z)();return l.createElement(p,(0,a.Z)({key:String(n)},e))}},9443:function(e,n,t){var a=(0,t(7294).createContext)(void 0);n.Z=a},266:function(e,n,t){t.r(n),t.d(n,{frontMatter:function(){return s},contentTitle:function(){return c},metadata:function(){return p},toc:function(){return d},default:function(){return b}});var a=t(3117),l=t(102),r=(t(7294),t(3905)),o=t(6396),u=t(8215),i=["components"],s={id:"installation",title:"Installation",sidebar_label:"Installation",slug:"/installation"},c=void 0,p={unversionedId:"installation",id:"version-3.x/installation",isDocsHomePage:!1,title:"Installation",description:"\x3c!-- when updating this file with a new version number, do a search and replace",source:"@site/versioned_docs/version-3.x/installation.mdx",sourceDirName:".",slug:"/installation",permalink:"/docs/installation",editUrl:"https://github.com/uber/h3/edit/master/website/docs/installation.mdx",tags:[],version:"3.x",frontMatter:{id:"installation",title:"Installation",sidebar_label:"Installation",slug:"/installation"},sidebar:"version-3.x/someSidebar",previous:{title:"Placekey",permalink:"/docs/comparisons/placekey"},next:{title:"Quick Start",permalink:"/docs/quickstart"}},d=[{value:"Package managers",id:"package-managers",children:[],level:2},{value:"Install from source",id:"install-from-source",children:[],level:2}],m={toc:d};function b(e){var n=e.components,t=(0,l.Z)(e,i);return(0,r.kt)("wrapper",(0,a.Z)({},m,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"We recommend using prebuilt bindings if they are available for your programming language. Bindings for Go, Java, JavaScript, Python, and others are available."),(0,r.kt)("h2",{id:"package-managers"},"Package managers"),(0,r.kt)(o.Z,{groupId:"env",defaultValue:"python",values:[{label:"Python",value:"python"},{label:"Java",value:"java"},{label:"JavaScript",value:"javascript"},{label:"Homebrew",value:"brew"}],mdxType:"Tabs"},(0,r.kt)(u.Z,{value:"python",mdxType:"TabItem"},(0,r.kt)("p",null,"Using ",(0,r.kt)("a",{parentName:"p",href:"https://pypi.org/project/h3/"},"PyPi"),", run:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"pip install h3\n")),(0,r.kt)("p",null,"Using ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/conda-forge/h3-py-feedstock"},"Conda"),", run:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre"},"conda config --add channels conda-forge\nconda install h3-py\n"))),(0,r.kt)(u.Z,{value:"java",mdxType:"TabItem"},(0,r.kt)("p",null,"Using Maven, add to your ",(0,r.kt)("inlineCode",{parentName:"p"},"pom.xml"),":"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-xml"},"<dependency>\n    <groupId>com.uber</groupId>\n    <artifactId>h3</artifactId>\n    <version>3.7.1</version>\n</dependency>\n")),(0,r.kt)("p",null,"Using Gradle, add to your build script:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-gradle"},'compile("com.uber:h3:3.7.1")\n'))),(0,r.kt)(u.Z,{value:"javascript",mdxType:"TabItem"},(0,r.kt)("p",null,"Using npm, run:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"npm install h3-js\n")),(0,r.kt)("p",null,"Using yarn, run:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"yarn add h3-js\n"))),(0,r.kt)(u.Z,{value:"brew",mdxType:"TabItem"},(0,r.kt)("p",null,"Using brew, run:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"brew install h3\n")),(0,r.kt)("p",null,"This installs the C library and filter applications."))),(0,r.kt)("h2",{id:"install-from-source"},"Install from source"),(0,r.kt)("p",null,"First, clone the repository or ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/uber/h3"},"download the source"),"\nand check out the latest release:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"git clone https://github.com/uber/h3.git\ncd h3\ngit checkout v$(<VERSION)\n")),(0,r.kt)("p",null,"You will need to install build dependencies for your operating system."),(0,r.kt)(o.Z,{groupId:"os",defaultValue:"macos",values:[{label:"mac OS",value:"macos"},{label:"alpine",value:"alpine"},{label:"Debian/Ubuntu",value:"debian"},{label:"Windows",value:"windows"},{label:"FreeBSD",value:"freebsd"}],mdxType:"Tabs"},(0,r.kt)(u.Z,{value:"macos",mdxType:"TabItem"},(0,r.kt)("p",null,"First make sure you ",(0,r.kt)("a",{parentName:"p",href:"http://osxdaily.com/2014/02/12/install-command-line-tools-mac-os-x/"},"have the developer tools installed")," and then run:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"# Installing the bare build requirements\nbrew install cmake\n# Installing useful tools for development\nbrew install clang-format lcov doxygen\n"))),(0,r.kt)(u.Z,{value:"alpine",mdxType:"TabItem"},(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"# Installing the bare build requirements\napk add cmake make gcc libtool musl-dev\n"))),(0,r.kt)(u.Z,{value:"debian",mdxType:"TabItem"},(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"# Installing the bare build requirements\nsudo apt install cmake make gcc libtool\n# Installing useful tools for development\nsudo apt install clang-format cmake-curses-gui lcov doxygen\n"))),(0,r.kt)(u.Z,{value:"windows",mdxType:"TabItem"},(0,r.kt)("p",null,"You will need to install CMake and Visual Studio, including the Visual C++ compiler."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"# Installing the bare build requirements\nsudo apt install cmake make gcc libtool\n# Installing useful tools for development\nsudo apt install clang-format cmake-curses-gui lcov doxygen\n"))),(0,r.kt)(u.Z,{value:"freebsd",mdxType:"TabItem"},(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"# Installing the build requirements\nsudo pkg install bash cmake gmake doxygen lcov\n")))),(0,r.kt)("p",null,"Next, you can build the library:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-bash"},"mkdir build\ncd build\ncmake ..\ncmake --build .\n")),(0,r.kt)("p",null,"Optionally, to run H3's test suite, run:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre"},"ctest\n")),(0,r.kt)("p",null,"By default, the filter applications are built when you build H3. You can confirm they are working by running:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre"},"./bin/geoToH3 --lat 14 --lon -42 --resolution 2\n")))}b.isMDXComponent=!0}}]);