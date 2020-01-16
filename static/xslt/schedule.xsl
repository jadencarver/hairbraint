<?xml version="1.0" encoding="UTF-8"?>

<xsl:stylesheet version="1.0"
  xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

<xsl:template match="/state">
  <style>
    body, text {
      font-family: "Lucida Grande", "Helvetica Neue", Helvetica, Verdana, sans-serif;
    }
    .track {
      fill: url(#track-pattern);
      stroke: #cccccc;
    }
    .blocks {
      fill: #ff9900;
      stroke: #000000;
    }
    .blocks .out-of-service {
      fill: #ccaaaa;
    }
    .blocks .confirmed {
      fill: #aaaaff;
    }
    .timeline text {
      font-size: 1em;
    }
    .providers {
      position: sticky;
      top: 0;
      margin-left: 58px;
      line-height: 2em;
    }
    .providers li {
      display: inline-block;
      width: 200px;
      text-align: center;
      background-color: rgba(200, 200, 200, 0.9);
      border: solid #aaaaaa 1pt;
    }
  </style>

  <ul class="providers">
    <xsl:for-each select="schedule/provider">
      <li><xsl:value-of select="name" /></li>
    </xsl:for-each>
  </ul>

  <xsl:variable name="jump">
    <xsl:for-each select="schedule/provider/blocks/block[ticket]/start/@offset">
      <xsl:sort data-type="number" order="ascending" />
      <xsl:if test="position()=1">
        <xsl:value-of select="." />
      </xsl:if>
    </xsl:for-each>
  </xsl:variable>

  <svg width="100%" height="144em">
    <defs>
      <pattern id="track-pattern" height="1.5em" width="200" patternUnits="userSpaceOnUse">
        <line x1="0" y1="0" x2="200" y2="0" stroke="#AAAAAA"></line>
      </pattern>
    </defs>

    <g class="timeline">
      <xsl:for-each select="/descendant::node()[position() &lt; 97]">
        <text x="90" y="1.25em" text-anchor="end">
          <xsl:if test="(position() + 3) * 15 &gt;= $jump and (position() +2) * 15 &lt; $jump">
            <xsl:attribute name="id">jump</xsl:attribute>
          </xsl:if>
          <xsl:attribute name="y">
            <xsl:value-of select="position() * 1.5 - 0.4"/>em
          </xsl:attribute>
          <xsl:variable name="hour24"><xsl:value-of select="floor((position() - 1) div 4)" /></xsl:variable>
          <xsl:variable name="minute">
            <xsl:value-of select="format-number((position() - 1) mod 4 * 15, '00')" />
          </xsl:variable>
          <xsl:if test="$hour24 = 0 or $hour24 = 12">12</xsl:if>
          <xsl:if test="$hour24 &lt; 12">
            <xsl:if test="$hour24 > 0"><xsl:value-of select="$hour24" /></xsl:if>:<xsl:value-of select="$minute" /> AM
          </xsl:if>
          <xsl:if test="$hour24 &gt;= 12">
            <xsl:if test="$hour24 > 12"><xsl:value-of select="$hour24 - 12" /></xsl:if>:<xsl:value-of select="$minute" /> PM
          </xsl:if>
        </text>
      </xsl:for-each>
    </g>

    <xsl:for-each select="schedule/provider">
      <xsl:variable name="x" select="(position() - 1) * 200 + 100" />
      <xsl:variable name="track">
        <rect class="track" y="0" width="200" height="100%">
          <xsl:attribute name="x">
            <xsl:value-of select="$x" />
          </xsl:attribute>
        </rect>
      </xsl:variable>
      <clipPath>
        <xsl:attribute name="id">track-<xsl:value-of select="position()" /></xsl:attribute>
        <xsl:copy-of select="$track" />
      </clipPath>
      <xsl:copy-of select="$track" />
      <g class="blocks">
        <xsl:attribute name="clip-path">url(#track-<xsl:value-of select="position()" /></xsl:attribute>
        <xsl:for-each select="blocks/block">
          <g>
            <xsl:attribute name="class">
              <xsl:choose>
                <xsl:when test="confirmed">confirmed</xsl:when>
                <xsl:when test="ticket">service</xsl:when>
                <xsl:otherwise>out-of-service</xsl:otherwise>
              </xsl:choose>
            </xsl:attribute>
            <rect width="200">
              <xsl:attribute name="x"><xsl:value-of select="$x" /></xsl:attribute>
              <xsl:attribute name="y"><xsl:value-of select="start/@offset div 10" />em</xsl:attribute>
              <xsl:attribute name="height"><xsl:value-of select="duration/@offset div 10" />em</xsl:attribute>
            </rect>
            <text>
              <xsl:attribute name="x"><xsl:value-of select="$x + 5" /></xsl:attribute>
              <xsl:attribute name="y"><xsl:value-of select="start/@offset div 10 + 1.1" />em</xsl:attribute>
              <xsl:value-of select="name" />
            </text>
          </g>
        </xsl:for-each>
      </g>
    </xsl:for-each>
  </svg>
  <script>
    document.addEventListener("DOMContentLoaded", function() {
      setTimeout(function() {
        var jump;
        if (jump = document.getElementById("jump")) {
          var jump_top = jump.getBoundingClientRect().top + scrollY;
          console.log(jump_top);
          window.scrollTo(0, jump_top);
          console.log(scrollY);
    }
    }, 100);
    });
  </script>

</xsl:template>

</xsl:stylesheet>
