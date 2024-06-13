import{r as s,R as H,c as ee,_ as V,j as x,F as N}from"./index-d21c8685.js";var te=Object.defineProperty,re=Object.defineProperties,ne=Object.getOwnPropertyDescriptors,T=Object.getOwnPropertySymbols,ae=Object.prototype.hasOwnProperty,oe=Object.prototype.propertyIsEnumerable,$=(e,t,r)=>t in e?te(e,t,{enumerable:!0,configurable:!0,writable:!0,value:r}):e[t]=r,ie=(e,t)=>{for(var r in t||(t={}))ae.call(t,r)&&$(e,r,t[r]);if(T)for(var r of T(t))oe.call(t,r)&&$(e,r,t[r]);return e},le=(e,t)=>re(e,ne(t));function se(e){const t=e.toLowerCase().split("+").map(o=>o.trim()),r={alt:t.includes("alt"),ctrl:t.includes("ctrl"),meta:t.includes("meta"),mod:t.includes("mod"),shift:t.includes("shift")},n=["alt","ctrl","meta","shift","mod"],a=t.find(o=>!n.includes(o));return le(ie({},r),{key:a})}function ue(e,t){const{alt:r,ctrl:n,meta:a,mod:o,shift:l,key:i}=e,{altKey:u,ctrlKey:c,metaKey:f,shiftKey:p,key:g}=t;if(r!==u)return!1;if(o){if(!c&&!f)return!1}else if(n!==c||a!==f)return!1;return l!==p?!1:!!(i&&(g.toLowerCase()===i.toLowerCase()||t.code.replace("Key","").toLowerCase()===i.toLowerCase()))}function ce(e){return t=>ue(se(e),t)}function W(e){return t=>{const r="nativeEvent"in t?t.nativeEvent:t;e.forEach(([n,a,o={preventDefault:!0}])=>{ce(n)(r)&&(o.preventDefault&&t.preventDefault(),a(r))})}}/**
 * @remix-run/router v1.8.0
 *
 * Copyright (c) Remix Software Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE.md file in the root directory of this source tree.
 *
 * @license MIT
 */function b(){return b=Object.assign?Object.assign.bind():function(e){for(var t=1;t<arguments.length;t++){var r=arguments[t];for(var n in r)Object.prototype.hasOwnProperty.call(r,n)&&(e[n]=r[n])}return e},b.apply(this,arguments)}var y;(function(e){e.Pop="POP",e.Push="PUSH",e.Replace="REPLACE"})(y||(y={}));function fe(e){e===void 0&&(e={});let{initialEntries:t=["/"],initialIndex:r,v5Compat:n=!1}=e,a;a=t.map((h,d)=>f(h,typeof h=="string"?null:h.state,d===0?"default":void 0));let o=u(r??a.length-1),l=y.Pop,i=null;function u(h){return Math.min(Math.max(h,0),a.length-1)}function c(){return a[o]}function f(h,d,m){d===void 0&&(d=null);let M=de(a?c().pathname:"/",h,d,m);return w(M.pathname.charAt(0)==="/","relative pathnames are not supported in memory history: "+JSON.stringify(h)),M}function p(h){return typeof h=="string"?h:pe(h)}return{get index(){return o},get action(){return l},get location(){return c()},createHref:p,createURL(h){return new URL(p(h),"http://localhost")},encodeLocation(h){let d=typeof h=="string"?C(h):h;return{pathname:d.pathname||"",search:d.search||"",hash:d.hash||""}},push(h,d){l=y.Push;let m=f(h,d);o+=1,a.splice(o,a.length,m),n&&i&&i({action:l,location:m,delta:1})},replace(h,d){l=y.Replace;let m=f(h,d);a[o]=m,n&&i&&i({action:l,location:m,delta:0})},go(h){l=y.Pop;let d=u(o+h),m=a[d];o=d,i&&i({action:l,location:m,delta:h})},listen(h){return i=h,()=>{i=null}}}}function v(e,t){if(e===!1||e===null||typeof e>"u")throw new Error(t)}function w(e,t){if(!e){typeof console<"u"&&console.warn(t);try{throw new Error(t)}catch{}}}function he(){return Math.random().toString(36).substr(2,8)}function de(e,t,r,n){return r===void 0&&(r=null),b({pathname:typeof e=="string"?e:e.pathname,search:"",hash:""},typeof t=="string"?C(t):t,{state:r,key:t&&t.key||n||he()})}function pe(e){let{pathname:t="/",search:r="",hash:n=""}=e;return r&&r!=="?"&&(t+=r.charAt(0)==="?"?r:"?"+r),n&&n!=="#"&&(t+=n.charAt(0)==="#"?n:"#"+n),t}function C(e){let t={};if(e){let r=e.indexOf("#");r>=0&&(t.hash=e.substr(r),e=e.substr(0,r));let n=e.indexOf("?");n>=0&&(t.search=e.substr(n),e=e.substr(0,n)),e&&(t.pathname=e)}return t}var k;(function(e){e.data="data",e.deferred="deferred",e.redirect="redirect",e.error="error"})(k||(k={}));function me(e,t,r){r===void 0&&(r="/");let n=typeof t=="string"?C(t):t,a=J(n.pathname||"/",r);if(a==null)return null;let o=z(e);ge(o);let l=null;for(let i=0;l==null&&i<o.length;++i)l=_e(o[i],Ie(a));return l}function z(e,t,r,n){t===void 0&&(t=[]),r===void 0&&(r=[]),n===void 0&&(n="");let a=(o,l,i)=>{let u={relativePath:i===void 0?o.path||"":i,caseSensitive:o.caseSensitive===!0,childrenIndex:l,route:o};u.relativePath.startsWith("/")&&(v(u.relativePath.startsWith(n),'Absolute route path "'+u.relativePath+'" nested under path '+('"'+n+'" is not valid. An absolute child route path ')+"must start with the combined path of all its parent routes."),u.relativePath=u.relativePath.slice(n.length));let c=E([n,u.relativePath]),f=r.concat(u);o.children&&o.children.length>0&&(v(o.index!==!0,"Index routes must not have child routes. Please remove "+('all child routes from route path "'+c+'".')),z(o.children,t,f,c)),!(o.path==null&&!o.index)&&t.push({path:c,score:Pe(c,o.index),routesMeta:f})};return e.forEach((o,l)=>{var i;if(o.path===""||!((i=o.path)!=null&&i.includes("?")))a(o,l);else for(let u of F(o.path))a(o,l,u)}),t}function F(e){let t=e.split("/");if(t.length===0)return[];let[r,...n]=t,a=r.endsWith("?"),o=r.replace(/\?$/,"");if(n.length===0)return a?[o,""]:[o];let l=F(n.join("/")),i=[];return i.push(...l.map(u=>u===""?o:[o,u].join("/"))),a&&i.push(...l),i.map(u=>e.startsWith("/")&&u===""?"/":u)}function ge(e){e.sort((t,r)=>t.score!==r.score?r.score-t.score:be(t.routesMeta.map(n=>n.childrenIndex),r.routesMeta.map(n=>n.childrenIndex)))}const ve=/^:\w+$/,ye=3,xe=2,Ee=1,Ce=10,Re=-2,A=e=>e==="*";function Pe(e,t){let r=e.split("/"),n=r.length;return r.some(A)&&(n+=Re),t&&(n+=xe),r.filter(a=>!A(a)).reduce((a,o)=>a+(ve.test(o)?ye:o===""?Ee:Ce),n)}function be(e,t){return e.length===t.length&&e.slice(0,-1).every((n,a)=>n===t[a])?e[e.length-1]-t[t.length-1]:0}function _e(e,t){let{routesMeta:r}=e,n={},a="/",o=[];for(let l=0;l<r.length;++l){let i=r[l],u=l===r.length-1,c=a==="/"?t:t.slice(a.length)||"/",f=Se({path:i.relativePath,caseSensitive:i.caseSensitive,end:u},c);if(!f)return null;Object.assign(n,f.params);let p=i.route;o.push({params:n,pathname:E([a,f.pathname]),pathnameBase:Me(E([a,f.pathnameBase])),route:p}),f.pathnameBase!=="/"&&(a=E([a,f.pathnameBase]))}return o}function Se(e,t){typeof e=="string"&&(e={path:e,caseSensitive:!1,end:!0});let[r,n]=we(e.path,e.caseSensitive,e.end),a=t.match(r);if(!a)return null;let o=a[0],l=o.replace(/(.)\/+$/,"$1"),i=a.slice(1);return{params:n.reduce((c,f,p)=>{if(f==="*"){let g=i[p]||"";l=o.slice(0,o.length-g.length).replace(/(.)\/+$/,"$1")}return c[f]=Le(i[p]||"",f),c},{}),pathname:o,pathnameBase:l,pattern:e}}function we(e,t,r){t===void 0&&(t=!1),r===void 0&&(r=!0),w(e==="*"||!e.endsWith("*")||e.endsWith("/*"),'Route path "'+e+'" will be treated as if it were '+('"'+e.replace(/\*$/,"/*")+'" because the `*` character must ')+"always follow a `/` in the pattern. To get rid of this warning, "+('please change the route path to "'+e.replace(/\*$/,"/*")+'".'));let n=[],a="^"+e.replace(/\/*\*?$/,"").replace(/^\/*/,"/").replace(/[\\.*+^$?{}|()[\]]/g,"\\$&").replace(/\/:(\w+)/g,(l,i)=>(n.push(i),"/([^\\/]+)"));return e.endsWith("*")?(n.push("*"),a+=e==="*"||e==="/*"?"(.*)$":"(?:\\/(.+)|\\/*)$"):r?a+="\\/*$":e!==""&&e!=="/"&&(a+="(?:(?=\\/|$))"),[new RegExp(a,t?void 0:"i"),n]}function Ie(e){try{return decodeURI(e)}catch(t){return w(!1,'The URL path "'+e+'" could not be decoded because it is is a malformed URL segment. This is probably due to a bad percent '+("encoding ("+t+").")),e}}function Le(e,t){try{return decodeURIComponent(e)}catch(r){return w(!1,'The value for the URL param "'+t+'" will not be decoded because'+(' the string "'+e+'" is a malformed URL segment. This is probably')+(" due to a bad percent encoding ("+r+").")),e}}function J(e,t){if(t==="/")return e;if(!e.toLowerCase().startsWith(t.toLowerCase()))return null;let r=t.endsWith("/")?t.length-1:t.length,n=e.charAt(r);return n&&n!=="/"?null:e.slice(r)||"/"}function Oe(e,t){t===void 0&&(t="/");let{pathname:r,search:n="",hash:a=""}=typeof e=="string"?C(e):e;return{pathname:r?r.startsWith("/")?r:je(r,t):t,search:Ne(n),hash:Te(a)}}function je(e,t){let r=t.replace(/\/+$/,"").split("/");return e.split("/").forEach(a=>{a===".."?r.length>1&&r.pop():a!=="."&&r.push(a)}),r.length>1?r.join("/"):"/"}function j(e,t,r,n){return"Cannot include a '"+e+"' character in a manually specified "+("`to."+t+"` field ["+JSON.stringify(n)+"].  Please separate it out to the ")+("`to."+r+"` field. Alternatively you may provide the full path as ")+'a string in <Link to="..."> and the router will parse it for you.'}function Ue(e){return e.filter((t,r)=>r===0||t.route.path&&t.route.path.length>0)}function Be(e,t,r,n){n===void 0&&(n=!1);let a;typeof e=="string"?a=C(e):(a=b({},e),v(!a.pathname||!a.pathname.includes("?"),j("?","pathname","search",a)),v(!a.pathname||!a.pathname.includes("#"),j("#","pathname","hash",a)),v(!a.search||!a.search.includes("#"),j("#","search","hash",a)));let o=e===""||a.pathname==="",l=o?"/":a.pathname,i;if(n||l==null)i=r;else{let p=t.length-1;if(l.startsWith("..")){let g=l.split("/");for(;g[0]==="..";)g.shift(),p-=1;a.pathname=g.join("/")}i=p>=0?t[p]:"/"}let u=Oe(a,i),c=l&&l!=="/"&&l.endsWith("/"),f=(o||l===".")&&r.endsWith("/");return!u.pathname.endsWith("/")&&(c||f)&&(u.pathname+="/"),u}const E=e=>e.join("/").replace(/\/\/+/g,"/"),Me=e=>e.replace(/\/+$/,"").replace(/^\/*/,"/"),Ne=e=>!e||e==="?"?"":e.startsWith("?")?e:"?"+e,Te=e=>!e||e==="#"?"":e.startsWith("#")?e:"#"+e;function $e(e){return e!=null&&typeof e.status=="number"&&typeof e.statusText=="string"&&typeof e.internal=="boolean"&&"data"in e}const q=["post","put","patch","delete"];new Set(q);const We=["get",...q];new Set(We);/**
 * React Router v6.15.0
 *
 * Copyright (c) Remix Software Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE.md file in the root directory of this source tree.
 *
 * @license MIT
 */function _(){return _=Object.assign?Object.assign.bind():function(e){for(var t=1;t<arguments.length;t++){var r=arguments[t];for(var n in r)Object.prototype.hasOwnProperty.call(r,n)&&(e[n]=r[n])}return e},_.apply(this,arguments)}const B=s.createContext(null),ke=s.createContext(null),I=s.createContext(null),L=s.createContext(null),R=s.createContext({outlet:null,matches:[],isDataRoute:!1}),G=s.createContext(null);function O(){return s.useContext(L)!=null}function Q(){return O()||v(!1),s.useContext(L).location}function X(e){s.useContext(I).static||s.useLayoutEffect(e)}function dt(){let{isDataRoute:e}=s.useContext(R);return e?Ze():Ae()}function Ae(){O()||v(!1);let e=s.useContext(B),{basename:t,navigator:r}=s.useContext(I),{matches:n}=s.useContext(R),{pathname:a}=Q(),o=JSON.stringify(Ue(n).map(u=>u.pathnameBase)),l=s.useRef(!1);return X(()=>{l.current=!0}),s.useCallback(function(u,c){if(c===void 0&&(c={}),!l.current)return;if(typeof u=="number"){r.go(u);return}let f=Be(u,JSON.parse(o),a,c.relative==="path");e==null&&t!=="/"&&(f.pathname=f.pathname==="/"?t:E([t,f.pathname])),(c.replace?r.replace:r.push)(f,c.state,c)},[t,r,o,a,e])}function De(e,t){return Ke(e,t)}function Ke(e,t,r){O()||v(!1);let{navigator:n}=s.useContext(I),{matches:a}=s.useContext(R),o=a[a.length-1],l=o?o.params:{};o&&o.pathname;let i=o?o.pathnameBase:"/";o&&o.route;let u=Q(),c;if(t){var f;let m=typeof t=="string"?C(t):t;i==="/"||(f=m.pathname)!=null&&f.startsWith(i)||v(!1),c=m}else c=u;let p=c.pathname||"/",g=i==="/"?p:p.slice(i.length)||"/",h=me(e,{pathname:g}),d=qe(h&&h.map(m=>Object.assign({},m,{params:Object.assign({},l,m.params),pathname:E([i,n.encodeLocation?n.encodeLocation(m.pathname).pathname:m.pathname]),pathnameBase:m.pathnameBase==="/"?i:E([i,n.encodeLocation?n.encodeLocation(m.pathnameBase).pathname:m.pathnameBase])})),a,r);return t&&d?s.createElement(L.Provider,{value:{location:_({pathname:"/",search:"",hash:"",state:null,key:"default"},c),navigationType:y.Pop}},d):d}function Ve(){let e=Ye(),t=$e(e)?e.status+" "+e.statusText:e instanceof Error?e.message:JSON.stringify(e),r=e instanceof Error?e.stack:null,a={padding:"0.5rem",backgroundColor:"rgba(200,200,200, 0.5)"},o=null;return s.createElement(s.Fragment,null,s.createElement("h2",null,"Unexpected Application Error!"),s.createElement("h3",{style:{fontStyle:"italic"}},t),r?s.createElement("pre",{style:a},r):null,o)}const ze=s.createElement(Ve,null);class Fe extends s.Component{constructor(t){super(t),this.state={location:t.location,revalidation:t.revalidation,error:t.error}}static getDerivedStateFromError(t){return{error:t}}static getDerivedStateFromProps(t,r){return r.location!==t.location||r.revalidation!=="idle"&&t.revalidation==="idle"?{error:t.error,location:t.location,revalidation:t.revalidation}:{error:t.error||r.error,location:r.location,revalidation:t.revalidation||r.revalidation}}componentDidCatch(t,r){console.error("React Router caught the following error during render",t,r)}render(){return this.state.error?s.createElement(R.Provider,{value:this.props.routeContext},s.createElement(G.Provider,{value:this.state.error,children:this.props.component})):this.props.children}}function Je(e){let{routeContext:t,match:r,children:n}=e,a=s.useContext(B);return a&&a.static&&a.staticContext&&(r.route.errorElement||r.route.ErrorBoundary)&&(a.staticContext._deepestRenderedBoundaryId=r.route.id),s.createElement(R.Provider,{value:t},n)}function qe(e,t,r){var n;if(t===void 0&&(t=[]),r===void 0&&(r=null),e==null){var a;if((a=r)!=null&&a.errors)e=r.matches;else return null}let o=e,l=(n=r)==null?void 0:n.errors;if(l!=null){let i=o.findIndex(u=>u.route.id&&(l==null?void 0:l[u.route.id]));i>=0||v(!1),o=o.slice(0,Math.min(o.length,i+1))}return o.reduceRight((i,u,c)=>{let f=u.route.id?l==null?void 0:l[u.route.id]:null,p=null;r&&(p=u.route.errorElement||ze);let g=t.concat(o.slice(0,c+1)),h=()=>{let d;return f?d=p:u.route.Component?d=s.createElement(u.route.Component,null):u.route.element?d=u.route.element:d=i,s.createElement(Je,{match:u,routeContext:{outlet:i,matches:g,isDataRoute:r!=null},children:d})};return r&&(u.route.ErrorBoundary||u.route.errorElement||c===0)?s.createElement(Fe,{location:r.location,revalidation:r.revalidation,component:p,error:f,children:h(),routeContext:{outlet:null,matches:g,isDataRoute:!0}}):h()},null)}var Y=function(e){return e.UseBlocker="useBlocker",e.UseRevalidator="useRevalidator",e.UseNavigateStable="useNavigate",e}(Y||{}),S=function(e){return e.UseBlocker="useBlocker",e.UseLoaderData="useLoaderData",e.UseActionData="useActionData",e.UseRouteError="useRouteError",e.UseNavigation="useNavigation",e.UseRouteLoaderData="useRouteLoaderData",e.UseMatches="useMatches",e.UseRevalidator="useRevalidator",e.UseNavigateStable="useNavigate",e.UseRouteId="useRouteId",e}(S||{});function Ge(e){let t=s.useContext(B);return t||v(!1),t}function Qe(e){let t=s.useContext(ke);return t||v(!1),t}function Xe(e){let t=s.useContext(R);return t||v(!1),t}function Z(e){let t=Xe(),r=t.matches[t.matches.length-1];return r.route.id||v(!1),r.route.id}function Ye(){var e;let t=s.useContext(G),r=Qe(S.UseRouteError),n=Z(S.UseRouteError);return t||((e=r.errors)==null?void 0:e[n])}function Ze(){let{router:e}=Ge(Y.UseNavigateStable),t=Z(S.UseNavigateStable),r=s.useRef(!1);return X(()=>{r.current=!0}),s.useCallback(function(a,o){o===void 0&&(o={}),r.current&&(typeof a=="number"?e.navigate(a):e.navigate(a,_({fromRouteId:t},o)))},[e,t])}const He="startTransition",D=H[He];function et(e){let{basename:t,children:r,initialEntries:n,initialIndex:a,future:o}=e,l=s.useRef();l.current==null&&(l.current=fe({initialEntries:n,initialIndex:a,v5Compat:!0}));let i=l.current,[u,c]=s.useState({action:i.action,location:i.location}),{v7_startTransition:f}=o||{},p=s.useCallback(g=>{f&&D?D(()=>c(g)):c(g)},[c,f]);return s.useLayoutEffect(()=>i.listen(p),[i,p]),s.createElement(rt,{basename:t,children:r,location:u.location,navigationType:u.action,navigator:i})}function tt(e){v(!1)}function rt(e){let{basename:t="/",children:r=null,location:n,navigationType:a=y.Pop,navigator:o,static:l=!1}=e;O()&&v(!1);let i=t.replace(/^\/*/,"/"),u=s.useMemo(()=>({basename:i,navigator:o,static:l}),[i,o,l]);typeof n=="string"&&(n=C(n));let{pathname:c="/",search:f="",hash:p="",state:g=null,key:h="default"}=n,d=s.useMemo(()=>{let m=J(c,i);return m==null?null:{location:{pathname:m,search:f,hash:p,state:g,key:h},navigationType:a}},[i,c,f,p,g,h,a]);return d==null?null:s.createElement(I.Provider,{value:u},s.createElement(L.Provider,{children:r,value:d}))}function pt(e){let{children:t,location:r}=e;return De(U(t),r)}new Promise(()=>{});function U(e,t){t===void 0&&(t=[]);let r=[];return s.Children.forEach(e,(n,a)=>{if(!s.isValidElement(n))return;let o=[...t,a];if(n.type===s.Fragment){r.push.apply(r,U(n.props.children,o));return}n.type!==tt&&v(!1),!n.props.index||!n.props.children||v(!1);let l={id:n.props.id||o.join("-"),caseSensitive:n.props.caseSensitive,element:n.props.element,Component:n.props.Component,index:n.props.index,path:n.props.path,loader:n.props.loader,action:n.props.action,errorElement:n.props.errorElement,ErrorBoundary:n.props.ErrorBoundary,hasErrorBoundary:n.props.ErrorBoundary!=null||n.props.errorElement!=null,shouldRevalidate:n.props.shouldRevalidate,handle:n.props.handle,lazy:n.props.lazy};n.props.children&&(l.children=U(n.props.children,o)),r.push(l)}),r}const[mt,P]=ee("mantine-spotlight"),nt=P("open"),at=P("close"),ot=P("toggle"),it=P("triggerAction"),lt=P("registerActions"),st=P("removeActions"),K={open:nt,close:at,toggle:ot,triggerAction:it,registerActions:lt,removeActions:st},ut=s.lazy(()=>V(()=>import("./Spotlight-e6df0bd3.js"),["assets/Spotlight-e6df0bd3.js","assets/index-d21c8685.js","assets/index-ccdf20da.css","assets/Text-1ac0e24d.js","assets/group-options-d9ed7c16.js","assets/TextInput-74649fc3.js","assets/extends-98964cd2.js","assets/createReactComponent-211b1769.js","assets/IconSearch-7964bd56.js","assets/IconPlus-7159126f.js"])),ct=s.lazy(()=>V(()=>import("./routes-6d5779ad.js"),["assets/routes-6d5779ad.js","assets/index-d21c8685.js","assets/index-ccdf20da.css"]));function ft(e){return s.useEffect(()=>(document.addEventListener("keydown",W([["mod + /",()=>K.open()]])),()=>{document.removeEventListener("keydown",W([["mode + /",()=>K.open()]]))}),[]),x.jsx(et,{initialEntries:["/"],initialIndex:0,children:x.jsx(s.Suspense,{fallback:x.jsx(N,{}),children:x.jsx(ut,{children:x.jsx(s.Suspense,{fallback:x.jsx(N,{}),children:x.jsx(ct,{...e})})})})})}const gt=Object.freeze(Object.defineProperty({__proto__:null,default:ft},Symbol.toStringTag,{value:"Module"}));export{gt as M,pt as R,dt as a,tt as b,W as c,ce as g,mt as u};