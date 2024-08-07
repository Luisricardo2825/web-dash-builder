use crate::builder::html;

pub fn get() -> String {
  html! {
    <%@ page language="java" contentType="text/html; charset=ISO-8859-1" pageEncoding="UTF-8" isELIgnored ="false"%>
    <!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
    <%@ taglib prefix="snk" uri="/WEB-INF/tld/sankhyaUtil.tld" %>
    <%@ page import="br.com.sankhya.modelcore.util.EntityFacadeFactory" %>
  }
}
