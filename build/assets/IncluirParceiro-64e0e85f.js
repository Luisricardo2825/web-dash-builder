import{i as f}from"./App-66b902bf.js";import"./index-d21c8685.js";import"./createReactComponent-211b1769.js";import"./Text-1ac0e24d.js";import"./events-73dbd6d4.js";async function F({user:o,adresses:e}){var E,u,a;const n=(E=o.CGC_CPF)==null?void 0:E.replace(/[^0-9]/g,""),i=o.CGC_CPF?{CGC_CPF:{$:n}}:{CLASSIFICMS:{$:"C"}},r=await S({CGC_CPF:n}),D=(u=e.filter(t=>t.checked))==null?void 0:u[0];if(console.log(D),!r){if(e.length<=0)throw new Error("Informe ao menos 1 endereço para cadastro");const t=e[0];if(!t)throw new Error("Endereço não encontrado");const{CEP:c,NUMEND:C}=t,d=c==null?void 0:c.replace(/[^0-9]/g,""),{data:s}=await A(d);if(!s)throw new Error("CEP não encontrado");const{responseBody:O}=s,{ceps:P}=O,{codBairro:p,codCid:N,codEnd:y}=P.enderecos,l=await f("/mge/service.sbr?serviceName=CRUDServiceProvider.saveRecord&outputType=json",{body:{serviceName:"CRUDServiceProvider.saveRecord",requestBody:{dataSet:{rootEntity:"Parceiro",includePresentationFields:"S",dataRow:{localFields:{TIPPESSOA:{$:o.tipoPessoa},NOMEPARC:{$:o.nomeParc},CODCID:{$:N},CODBAI:{$:p},CODEND:{$:y},ATIVO:{$:"S"},CEP:{$:d},NUMEND:{$:C},EMAIL:{$:o.email},CLIENTE:{$:"S"},...i}},entity:{fieldset:{list:"CODPARC,TIPPESSOA,NOMEPARC,CODCID,ATIVO,CLIENTE,CLASSIFICMS"}}}}},method:"POST"});if(l.ok)return(a=l.data)==null?void 0:a.responseBody.entities.entity.CODPARC.$;throw new Error(l.message)}if(e.length>0){const t=e.slice(1,e.length);for(const c of t){const{CEP:C,NUMEND:d}=c,s=(C==null?void 0:C.replace(/[^0-9]/g,""))||"",{data:O}=await A(s);if(!O)throw new Error("CEP não encontrado");const{responseBody:P}=O,{ceps:p}=P,{codBairro:N,codCid:y,codEnd:l}=p.enderecos;if(!await T({codparc:r,cep:s,numend:d||""})&&c.CODCONTATO!=="0"){const m=await w({codbai:N,codcid:y,codend:l,codparc:r,nomecontato:"Endereço",numend:String(d),cep:s});m&&console.log("Contao incluido:",m)}}}return r}async function S({CGC_CPF:o}){const e=`SELECT CODPARC FROM TGFPAR WHERE CGC_CPF = '${o}'`,n=await window.executeQueryAsync(e,[]);return n.length>0?n[0].CODPARC:null}async function A(o){const e=await f("/mge/service.sbr?serviceName=PesquisaCepSP.obterDadosDoCep&outputType=json",{method:"POST",body:{serviceName:"PesquisaCepSP.obterDadosDoCep",requestBody:{cep:{$:o}}}});if(e.ok)return e;throw console.log(e.message),new Error(e.message)}async function w({codparc:o,codcid:e,codbai:n,codend:i,nomecontato:r,numend:D,cep:E}){var a;const u={serviceName:"CRUDServiceProvider.saveRecord",requestBody:{dataSet:{rootEntity:"Contato",includePresentationFields:"S",dataRow:{localFields:{CODPARC:{$:o},CODCID:{$:e},CODBAI:{$:n},CODEND:{$:i},NOMECONTATO:{$:r},NUMEND:{$:D},CEP:{$:E}}},entity:{fieldset:{list:"CODPARC,CODCID,CODBAI,CODEND,NOMECONTATO,NUMEND"}}}}};try{const t=await f("/mge/service.sbr?serviceName=CRUDServiceProvider.saveRecord&outputType=json",{method:"POST",body:u});return t.ok?((a=t.data)==null?void 0:a.responseBody.entities.entity.CODCONTATO.$)||"":null}catch(t){return console.log(t),null}}async function T({codparc:o,cep:e,numend:n}){const i=`SELECT CODCONTATO FROM TGFCTT WHERE CODPARC = '${o}' AND CEP = '${e}' AND NUMEND = '${n}'`;return(await window.executeQueryAsync(i,[])).length>0}export{F as default};
