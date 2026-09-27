---
title: Places I've Been - Py4_
slug: places
description: Places I've traveled around the world.
layout: places
format: html
nav_active: places
extra_css: places.css
extra_js: places.js
---

<div class="map-container art" style="position:relative">

    <h1 class="im tu bb">Countries I've Visited</h1>

<div class="country-label" id="tooltip"></div>
<svg class="world-map" viewBox="0 0 1000 500" xmlns="http://www.w3.org/2000/svg">
<!-- Ocean background -->
<rect x="0" y="0" width="1000" height="500" fill="#0066cc"/>
<!-- North America - wider at top, narrower at bottom -->
<polygon points="50,100 180,80 200,120 190,180 170,220 130,240 90,200 60,150" fill="#000" stroke="#fff" stroke-width="4"/>

<!-- South America - narrower, tilted -->
<polygon points="140,260 170,250 185,280 190,330 180,380 160,400 135,390 125,340 130,290" fill="#000" stroke="#fff" stroke-width="4"/>

<!-- Europe - small, irregular -->
<polygon points="400,110 450,100 480,120 470,150 440,160 410,145" fill="#000" stroke="#fff" stroke-width="4"/>

<!-- Africa - wider at top, pointed bottom -->
<polygon points="420,180 500,170 540,210 545,280 530,340 490,370 450,360 430,320 415,260 418,220" fill="#000" stroke="#fff" stroke-width="4"/>

<!-- Asia - large, irregular mass -->
<polygon points="500,70 620,60 720,80 780,120 790,170 770,210 720,240 650,250 580,230 540,200 520,160 505,120" fill="#000" stroke="#fff" stroke-width="4"/>

<!-- Australia - horizontal oval shape -->
<polygon points="720,300 800,290 850,320 840,360 790,380 730,375 710,340" fill="#000" stroke="#fff" stroke-width="4"/>

<!-- Visited markers -->
<!-- Canada -->
<circle cx="130" cy="140" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Canada"/>
<!-- United States -->
<circle cx="140" cy="175" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="United States"/>
<!-- Mexico -->
<circle cx="110" cy="210" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Mexico"/>
<!-- Costa Rica -->
<circle cx="125" cy="245" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Costa Rica"/>
<!-- Colombia -->
<circle cx="150" cy="270" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Colombia"/>
<!-- UK -->
<circle cx="430" cy="125" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="United Kingdom"/>
<!-- Netherlands -->
<circle cx="445" cy="115" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Netherlands"/>
<!-- Germany -->
<circle cx="460" cy="125" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Germany"/>
<!-- Spain -->
<circle cx="415" cy="140" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Spain"/>
<!-- Portugal -->
<circle cx="395" cy="145" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Portugal"/>
<!-- Azores Islands -->
<circle cx="350" cy="135" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Azores Islands"/>
<!-- Turkey -->
<circle cx="530" cy="155" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Turkey"/>
<!-- Lebanon -->
<circle cx="510" cy="165" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Lebanon"/>
<!-- Iran -->
<circle cx="620" cy="160" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Iran"/>
<!-- Hong Kong -->
<circle cx="750" cy="180" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Hong Kong"/>
<!-- Malaysia -->
<circle cx="730" cy="210" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Malaysia"/>
<!-- Singapore -->
<circle cx="735" cy="220" r="12" fill="#f00" stroke="#000" stroke-width="3" style="cursor:pointer" class="country-marker" data-country="Singapore"/>

<!-- Labels -->
<text x="125" y="70" font-family="Impact,sans-serif" font-size="18" font-weight="900" fill="#000" text-anchor="middle">N. AMERICA</text>
<text x="155" y="415" font-family="Impact,sans-serif" font-size="18" font-weight="900" fill="#000" text-anchor="middle">S. AMERICA</text>
<text x="445" y="95" font-family="Impact,sans-serif" font-size="18" font-weight="900" fill="#000" text-anchor="middle">EUROPE</text>
<text x="470" y="390" font-family="Impact,sans-serif" font-size="18" font-weight="900" fill="#000" text-anchor="middle">AFRICA</text>
<text x="640" y="55" font-family="Impact,sans-serif" font-size="18" font-weight="900" fill="#000" text-anchor="middle">ASIA</text>
<text x="775" y="285" font-family="Impact,sans-serif" font-size="18" font-weight="900" fill="#000" text-anchor="middle">AUSTRALIA</text>
</svg>

</div>
