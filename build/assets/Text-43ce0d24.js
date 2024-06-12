import{b as E,m as W,r as z,u as D,e as I,B,n as F}from"./index-24cbb48d.js";var H=Object.defineProperty,R=Object.defineProperties,A=Object.getOwnPropertyDescriptors,y=Object.getOwnPropertySymbols,L=Object.prototype.hasOwnProperty,V=Object.prototype.propertyIsEnumerable,w=(t,e,r)=>e in t?H(t,e,{enumerable:!0,configurable:!0,writable:!0,value:r}):t[e]=r,l=(t,e)=>{for(var r in e||(e={}))L.call(e,r)&&w(t,r,e[r]);if(y)for(var r of y(e))V.call(e,r)&&w(t,r,e[r]);return t},q=(t,e)=>R(t,A(e));function G({underline:t,strikethrough:e}){const r=[];return t&&r.push("underline"),e&&r.push("line-through"),r.length>0?r.join(" "):"none"}function J({theme:t,color:e}){return e==="dimmed"?t.fn.dimmed():typeof e=="string"&&(e in t.colors||e.split(".")[0]in t.colors)?t.fn.variant({variant:"filled",color:e}).background:e||"inherit"}function K(t){return typeof t=="number"?{overflow:"hidden",textOverflow:"ellipsis",display:"-webkit-box",WebkitLineClamp:t,WebkitBoxOrient:"vertical"}:null}function M({theme:t,truncate:e}){return e==="start"?{overflow:"hidden",textOverflow:"ellipsis",whiteSpace:"nowrap",direction:t.dir==="ltr"?"rtl":"ltr",textAlign:t.dir==="ltr"?"right":"left"}:e?{overflow:"hidden",textOverflow:"ellipsis",whiteSpace:"nowrap"}:null}var Q=E((t,{color:e,lineClamp:r,truncate:n,inline:f,inherit:i,underline:p,gradient:c,weight:u,transform:a,align:d,strikethrough:g,italic:v},{size:o})=>{const _=t.fn.variant({variant:"gradient",gradient:c});return{root:q(l(l(l(l({},t.fn.fontStyles()),t.fn.focusStyles()),K(r)),M({theme:t,truncate:n})),{color:J({color:e,theme:t}),fontFamily:i?"inherit":t.fontFamily,fontSize:i||o===void 0?"inherit":W({size:o,sizes:t.fontSizes}),lineHeight:i?"inherit":f?1:t.lineHeight,textDecoration:G({underline:p,strikethrough:g}),WebkitTapHighlightColor:"transparent",fontWeight:i?"inherit":u,textTransform:a,textAlign:d,fontStyle:v?"italic":void 0}),gradient:{backgroundImage:_.background,WebkitBackgroundClip:"text",WebkitTextFillColor:"transparent"}}});const U=Q;var X=Object.defineProperty,s=Object.getOwnPropertySymbols,x=Object.prototype.hasOwnProperty,P=Object.prototype.propertyIsEnumerable,O=(t,e,r)=>e in t?X(t,e,{enumerable:!0,configurable:!0,writable:!0,value:r}):t[e]=r,Y=(t,e)=>{for(var r in e||(e={}))x.call(e,r)&&O(t,r,e[r]);if(s)for(var r of s(e))P.call(e,r)&&O(t,r,e[r]);return t},Z=(t,e)=>{var r={};for(var n in t)x.call(t,n)&&e.indexOf(n)<0&&(r[n]=t[n]);if(t!=null&&s)for(var n of s(t))e.indexOf(n)<0&&P.call(t,n)&&(r[n]=t[n]);return r};const tt={variant:"text"},h=z.forwardRef((t,e)=>{const r=D("Text",tt,t),{className:n,size:f,weight:i,transform:p,color:c,align:u,variant:a,lineClamp:d,truncate:g,gradient:v,inline:o,inherit:_,underline:b,strikethrough:S,italic:T,classNames:et,styles:rt,unstyled:C,span:j,__staticSelector:k}=r,N=Z(r,["className","size","weight","transform","color","align","variant","lineClamp","truncate","gradient","inline","inherit","underline","strikethrough","italic","classNames","styles","unstyled","span","__staticSelector"]),{classes:m,cx:$}=U({color:c,lineClamp:d,truncate:g,inline:o,inherit:_,underline:b,strikethrough:S,italic:T,weight:i,transform:p,align:u,gradient:v},{unstyled:C,name:k||"Text",variant:a,size:f});return I.createElement(B,Y({ref:e,className:$(m.root,{[m.gradient]:a==="gradient"},n),component:j?"span":"div"},N))});h.displayName="@mantine/core/Text";const it=F(h);export{it as T};