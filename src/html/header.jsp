<%@ page language="java" contentType="text/html; charset=ISO-8859-1" pageEncoding="UTF-8" isELIgnored ="false"%>
<%@ page session="true" %>

<%@ taglib prefix="c"   uri="http://java.sun.com/jsp/jstl/core" %>


<% 
    br.com.sankhya.jape.EntityFacade dwfFacade = br.com.sankhya.modelcore.util.EntityFacadeFactory.getDWFFacade();
    br.com.sankhya.jape.dao.JdbcWrapper jdbcWrapper = dwfFacade.getJdbcWrapper();
    String databaseProductName = "unknown";
    if (jdbcWrapper.isOracle()){
        databaseProductName = "oracle";
    }
    else if (jdbcWrapper.isSQLServer()){
        databaseProductName = "mssql";
    }
    else if (jdbcWrapper.isMysql()){
        databaseProductName = "mysql";
    }
    jdbcWrapper.closeSession();
%>

<%
    String baseFolder = ((String) request.getAttribute("BASE_FOLDER"));
    baseFolder = baseFolder.replace("\\", "/");
%>