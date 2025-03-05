<%@ page language="java" contentType="text/html; charset=ISO-8859-1" pageEncoding="UTF-8" isELIgnored ="false"%>
<%@ taglib prefix="snk" uri="/WEB-INF/tld/sankhyaUtil.tld" %>
<%@ page import="br.com.sankhya.jape.EntityFacade" %>
<%@ page session="true" %>
<%@taglib uri="http://java.sun.com/jsp/jstl/core" prefix="c"%>
<%@ page import="br.com.sankhya.jape.dao.JdbcWrapper" %>
<%@ page import="br.com.sankhya.modelcore.util.EntityFacadeFactory" %>

<% 
    EntityFacade dwfFacade = EntityFacadeFactory.getDWFFacade();
    JdbcWrapper jdbcWrapper = dwfFacade.getJdbcWrapper();
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