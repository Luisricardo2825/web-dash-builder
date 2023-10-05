import{r as i}from"./index.ed373d49.js";var s={exports:{}},n={};/**
 * @license React
 * react-jsx-runtime.production.min.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var f=i,c=Symbol.for("react.element"),d=Symbol.for("react.fragment"),_=Object.prototype.hasOwnProperty,m=f.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED.ReactCurrentOwner,y={key:!0,ref:!0,__self:!0,__source:!0};function u(r,t,l){var e,o={},p=null,a=null;l!==void 0&&(p=""+l),t.key!==void 0&&(p=""+t.key),t.ref!==void 0&&(a=t.ref);for(e in t)_.call(t,e)&&!y.hasOwnProperty(e)&&(o[e]=t[e]);if(r&&r.defaultProps)for(e in t=r.defaultProps,t)o[e]===void 0&&(o[e]=t[e]);return{$$typeof:c,type:r,key:p,ref:a,props:o,_owner:m.current}}n.Fragment=d;n.jsx=u;n.jsxs=u;s.exports=n;var x=s.exports;function v(){return x.jsx("button",{onClick:()=>{SanPopup.open({title:"Quantidade Apontada",templateUrl:"html5/ApontamentoLeitor/popup/popupQtdApontada.tpl.html",controller:"popupQtdApontadaController",controllerAs:"ctrl",type:"brand",size:"alert",grayBG:!0,okBtnLabel:i18n("Apontar"),showBtnNo:!1,showIconClose:!1}).result.then(function(r){console.log(r)})},children:"Testar botão react"})}export{v as default};
