import "./types";
import * as LightweightCharts from 'lightweight-charts' ;
import {BarSeriesPartialOptions, ChartOptions, IChartApi, ISeriesApi, SeriesMarker, Time} from "lightweight-charts";
import {ITimeValue} from "./types";

var width = 1600;

(function() {
    width =  window.innerWidth * 0.98;
})();
function $$(id) {
    return document.getElementById(id);
}

// Make Bar Chart
function getChartCfg(width,height): ChartOptions {
    return {
        width: width,
        height: height,
        layout: {
            backgroundColor: '#ffffff',
            textColor: 'rgba(33, 56, 77, 1)',
        },
        crosshair: {
            mode: LightweightCharts.CrosshairMode.Normal,
        },
        rightPriceScale: {
            borderColor: 'rgba(197, 203, 206, 1)',
            drawTicks: false, //dosnet works??
        },
        timeScale: {
            borderColor: 'rgba(197, 203, 206, 1)',
            secondsVisible: true,
            timeVisible: true,
            tickMarkFormatter: (time, tickMarkType, locale) => {
                var d = new Date(time * 1000);
                let day = d.getUTCDate();
                let hour = d.getUTCHours();
                let min = d.getUTCMinutes();
                let sec = d.getUTCSeconds();
                var str =  "" + day  + "  " + hour+":"+min+":"+sec ;
                return str
            },
        },
    }
}

export function buildBarChart(p: {el: HTMLElement,height:number,black:boolean,ohlc:any,markers?: SeriesMarker<Time>[]}) :IChartApi {
    var width =  window.innerWidth * 0.98;
    var chart = LightweightCharts.createChart(p.el, getChartCfg(width, p.height));
    let barSeriesOpt :BarSeriesPartialOptions = {
        thinBars: false,
        priceFormat: {
            minMove: 0.00001,
            precision: 5,
        },
    };

    if (p.black) {
        barSeriesOpt.downColor = '#000';
        barSeriesOpt.upColor = '#000';
    }

    var barSeries = chart.addBarSeries(barSeriesOpt);
    barSeries.setData(p.ohlc);
    if(p.markers != undefined){
        barSeries.setMarkers(p.markers);
    }
    return chart;
}

export function trendChannelChart(chart: IChartApi, d) {
    var bullLine = chart.addLineSeries({
        color: 'rgb(34,215,104)',
        lineWidth: 1,
    });
    bullLine.setData(d.bull_line);

    var bearLine = chart.addLineSeries({
        color: 'rgb(215,49,68)',
        lineWidth: 1,
    });
    bearLine.setData(d.bear_line);
}

export function rpiOverIndicator(chart: IChartApi, d) {
    var highLine = chart.addLineSeries({
        color: 'rgb(255,145,0)',
        lineWidth: 1.5,
    });
    highLine.setData(d.rpi_high);

    var lowLine = chart.addLineSeries({
        color: 'rgb(255,145,0)',
        lineWidth: 1.5,
    });
    lowLine.setData(d.rpi_low);
}

// Simple
export function simpleLineOver(chart: IChartApi, d, color?: string,lineWidth?:number) {
    if (color == undefined){
        color = 'rgb(11,77,229)';
    }
    if (lineWidth == undefined){
        lineWidth =1;
    }
    var line = chart.addLineSeries({
        color: color,
        lineWidth: lineWidth,
    });
    line.setData(d);
}

// Sub indicators
export function macdChart(el,d) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
        timeScale: {
            visible: false,
        }
    });
    var lineMacd = chart.addLineSeries({
        color: 'rgb(17,66,148)',
        lineWidth: 1,
    });
    lineMacd.setData(d.macd_macd);
    var lineSignal = chart.addLineSeries({
        color: 'rgb(203,66,66)',
        lineWidth: 1,
    });
    lineSignal.setData(d.macd_signal);
    var histogram = chart.addHistogramSeries({
        color: 'rgb(105,102,102)',
        lineWidth: 2,
        // priceScaleId: 'left',
    });
    histogram.setData(d.macd_histogram);
    return chart;
}

export function scoreChart(el,d) {
    // Chart 2 - Scores
    var chart2TV = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
    });

    var scoreBull = chart2TV.addHistogramSeries({
        color: 'rgba(34,140,74,0.69)',
        lineWidth: 1,
    });
    scoreBull.setData(d.score_bull);

    var scoreBear = chart2TV.addHistogramSeries({
        color: 'rgba(224,49,68,0.68)',
        lineWidth: 1,
    });
    scoreBear.setData(d.score_bear);

    var scoreDiff = chart2TV.addLineSeries({
        color: 'rgb(1,5,2)',
        lineWidth: 2,
    });
    scoreDiff.setData(d.score_diff);
    return chart2TV;
}

export function trendDirectionChart(el,d) {
    // Chart 2 - Scores
    var chart2TV = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
    });

    var scoreBull = chart2TV.addHistogramSeries({
        color: 'rgba(34,140,74,0.69)',
        lineWidth: 1,
    });
    scoreBull.setData(d.td_plus);

    var scoreBear = chart2TV.addHistogramSeries({
        color: 'rgba(224,49,68,0.68)',
        lineWidth: 1,
    });
    scoreBear.setData(d.td_minus);

    var scoreDiff = chart2TV.addLineSeries({
        color: 'rgb(1,5,2)',
        lineWidth: 2,
    });
    scoreDiff.setData(d.td_diff);
    return chart2TV;
}

export function mdi(el,d) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
        timeScale: {
            visible: false,
        }
    });
    var line1 = chart.addLineSeries({
        color: 'rgb(39,145,77)',
        lineWidth: 1,
    });
    line1.setData(d.dmi_plus);
    var line2 = chart.addLineSeries({
        color: 'rgb(203,66,66)',
        lineWidth: 1,
    });
    line2.setData(d.dmi_minus);
    var line3 = chart.addLineSeries({
        color: 'rgb(105,102,102)',
        lineWidth: 2,
        // priceScaleId: 'left',
    });
    line3.setData(d.dmi_diff);
    return chart;
}

// Relative DC percenatge
export function relDc(el,d) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
        timeScale: {
            visible: false,
        }
    });
    var line1 = chart.addLineSeries({
        color: 'rgb(39,145,77)',
        lineWidth: 1,
    });
    line1.setData(d.rdc_med);
    var line2 = chart.addLineSeries({
        color: 'rgb(203,66,66)',
        lineWidth: 1,
    });
    line2.setData(d.rdc_big);
//     var line3 = chart.addLineSeries({
//         color: 'rgb(105,102,102)',
//         lineWidth: 2,
//         // priceScaleId: 'left',
//     });
//     line3.setData(d.dmi_diff);
    return chart;
}

// Relative DC
export function relDcHeight(el,d) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
        timeScale: {
            visible: false,
        }
    });
    var line1 = chart.addLineSeries({
        color: 'rgb(39,145,77)',
        lineWidth: 1,
    });
    line1.setData(d.rdc_med_height);
    var line2 = chart.addLineSeries({
        color: 'rgb(203,66,66)',
        lineWidth: 1,
    });
    line2.setData(d.rdc_big_height);
    return chart;
}

export function maMomChart(el,d) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
    });

    var scoreBull = chart.addHistogramSeries({
        color: 'rgba(21,71,166,0.69)',
        lineWidth: 1,
        priceFormat: {
            minMove: 0.00001,
            precision: 5,
        },
    });
    // scoreBull.setData(d.major.ma_mom);
    scoreBull.setData(d.major.ma_mom);

    var scoreBear = chart.addHistogramSeries({
        color: 'rgba(255,19,19,0.68)',
        lineWidth: 1,
        priceFormat: {
            minMove: 0.00001,
            precision: 5,
        },
    });
    scoreBear.setData(d.medium.ma_mom);

    return chart;
}

////////////////// Relative Price (RP) ///////////////////
export function relPrice(el,d) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
        timeScale: {
            visible: false,
        }
    });
    var line1 = chart.addLineSeries({
        color: 'rgb(39,145,77)',
        lineWidth: 1,
    });
    line1.setData(d.medium.rp_os_index);
    var line2 = chart.addLineSeries({
        color: 'rgb(203,66,66)',
        lineWidth: 1,
    });
    line2.setData(d.major.rp_os_index);
//     var line3 = chart.addLineSeries({
//         color: 'rgb(105,102,102)',
//         lineWidth: 2,
//         // priceScaleId: 'left',
//     });
//     line3.setData(d.dmi_diff);
    return chart;
}

//////////////////////////////////////////////////////////


////////////////// General Lines  ////////////////////////
// for stochi for example
export function threeLines(el,line1Data, line2Data,line3Data) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
        timeScale: {
            visible: false,
        }
    });
    var line1 = chart.addLineSeries({
        color: 'rgb(39,145,77)',
        lineWidth: 1,
    });
    line1.setData(line1Data);

    if(line2Data != undefined) {
        var line2 = chart.addLineSeries({
            color: 'rgb(203,66,66)',
            lineWidth: 1,
        });
        line2.setData(line2Data);
    }

    if(line3Data != undefined) {
        var line3 = chart.addLineSeries({
            color: 'rgb(105,102,102)',
            lineWidth: 2,
            // priceScaleId: 'left',
        });
        line3.setData(line3Data);
    }
    return chart;
}
//////////////////////////////////////////////////////////

export function onelineSubIndiacor(el,d :ITimeValue[] ) {
    var chart = LightweightCharts.createChart(el, {
        width: width,
        height: width/14,
        crosshair: {
            mode: 0
        },
        rightPriceScale: {
            width: 60
        },
    });

    var scoreBull = chart.addHistogramSeries({
        color: 'rgba(21,71,166,0.69)',
        lineWidth: 1,
        priceFormat: {
            minMove: 0.00001,
            precision: 5,
        },
    });
    scoreBull.setData(d);

    return chart;
}

//////////////////// LocalStorage fns ////////////////////////
function getStore(namespace:string,key:string,def?:any ) :any{
    var dbNs = localStorage[namespace];
    if(dbNs == undefined) {
        return def;
    }
    var db = JSON.parse(dbNs);
    var valStore = db[key];
    if(valStore == undefined) {
        return def;
    }
    return valStore;
}

function setStore(namespace:string,key:string,val:any ) {
    var dbNs = localStorage[namespace];
    if(dbNs == undefined) {
        dbNs = JSON.stringify({});
        localStorage[namespace] = dbNs;
    }
    var db = JSON.parse(dbNs);
    db[key]=val;
    localStorage[namespace] = JSON.stringify(db);
}

function getStoreDB(namespace:string) {
    var dbNs = localStorage[namespace];
    if(dbNs == undefined) {
        dbNs = JSON.stringify({});
        localStorage[namespace] = dbNs;
    }
    return JSON.parse(dbNs);
}

function getOrSetStore(namespace:string,key:string,val:any ):any {
    var storVal = getStore(namespace,key,undefined);
    if(storVal == undefined) {
        setStore(namespace,key,val);
        return val;
    }
    return storVal;
}

export function resetSubIndicatorsStorage(){
    // localStorage.clear();
    localStorage[INDI] = "{}";
    window.location.reload();
    // runCheckboxIndicatorsShowHide();
}
////////////////////////// End ///////////////////////////

const INDI = "INDI";
const OVERLY = "OVERLY";
export function makeNextIndi(name:string,visibleOrg:boolean,topHolder:boolean){
    var visible = getOrSetStore(INDI,name,visibleOrg);

    var checked_attr = "checked";
    if(visible == false){
        checked_attr = "";
    }
    // jQuery
    let check_txt = `<label class="label"><input type="checkbox" ${checked_attr} id="btn_${name}" data-name="${name}" onchange="checkboxChange(this)" class="checkbox"  > ${name} </input></label>`;
    $("form#form_sub_indicators").append(check_txt);

    var el = document.createElement("div");
    el.id = "chart_" + name;
    el.classList.add("chart");
    if(visible == false){
        el.style.display ="none";
    }

    var par = $$("top_indicators");
    if(topHolder == false){
        par = $$("bottom_indicators");
    }
    par.append(el);

    return el ;
}

// Change chart2 time visible when chart1 moves (major > medium)
export function syncCharts(chart1: IChartApi,chart2: IChartApi){
    chart1.timeScale().subscribeVisibleTimeRangeChange((t) => chart2.timeScale().setVisibleRange(t))
}

export function checkboxChartChange(){
    var chartMajorEl = document.getElementById("chart_major");
    var chartMediumEL = document.getElementById("chart_medium");
    var chartSmallEL = document.getElementById("chart_small");

    var major_check_btn = document.getElementById("major_check_btn");
    var medium_check_btn = document.getElementById("medium_check_btn");
    var small_check_btn = document.getElementById("small_check_btn");

    if(major_check_btn.checked) {
        chartMajorEl.style.display = "block";
    } else {
        chartMajorEl.style.display = "none";
    }

    if(medium_check_btn.checked) {
        chartMediumEL.style.display = "block";
    } else {
        chartMediumEL.style.display = "none";
    }

    if(small_check_btn.checked) {
        chartSmallEL.style.display = "block";
    } else {
        chartSmallEL.style.display = "none";
    }
}

export function checkboxChange(th: HTMLElement){
//     checkboxChartChange();
    if(th == undefined){
        return;
    }
    let nameKey = th.id.replace("btn_","");
    let valShow = getStore(INDI,nameKey);
    // Swap inidicator show in LocalStorge
    if(valShow == true){
        setStore(INDI,nameKey,false);
    } else{
        setStore(INDI,nameKey,true);
    }

    runCheckboxIndicatorsShowHide();
}

function runCheckboxIndicatorsShowHide() {
    for (const name in getStoreDB(INDI)) {
        var valBoll = getStore(INDI,name,false);
        let el = $$("btn_"+name);
        let chart_el = $$("chart_"+name);
        if(el != null && chart_el != null) {
            if(valBoll) {
                el.checked = true;
                chart_el.style.display = "block";
            } else {
                el.checked = false;
                chart_el.style.display = "none";
            }
        }
    }
}

export function hideAllSubIndicators(){
    for (const name in getStoreDB(INDI)) {
        setStore(INDI,name,false);
    }
    runCheckboxIndicatorsShowHide();
}

// Overly
export function getOverlyShow(name:string,visibleDef:boolean){
    return getOrSetStore(OVERLY,name,visibleDef);
}

export function checkboxOverlyChange(th: HTMLElement){
    if(th == undefined){
        return;
    }
    let nameKey = th.id.replace("btn_o_","");
    let valShow = getStore(OVERLY,nameKey);
    // Swap inidicator show in LocalStorge
    if(valShow == true){
        setStore(OVERLY,nameKey,false);
    } else{
        setStore(OVERLY,nameKey,true);
    }

    runCheckboxOverlyShowHide();
}

function runCheckboxOverlyShowHide() {
    for (const name in getStoreDB(OVERLY)) {
        var valBoll = getStore(OVERLY,name,false);
        let el = $$("btn_o_"+name);
        if(el != null) {
            if(valBoll) {
                el.checked = true;
            } else {
                el.checked = false;
            }
        }
    }
}

export function buildOverlyHtml(){
    for (const name in getStoreDB(OVERLY)) {
        var visible = getStore(OVERLY,name,false);
        var checked_attr = "checked";
        if(visible == false){
            checked_attr = "";
        }
        // jQuery
        let check_txt = `<label class="label"><input type="checkbox" ${checked_attr} id="btn_o_${name}" data-name="${name}" onchange="checkboxOverlyChange(this)" class="checkbox"  > ${name} </input></label>`;
        $("form#form_overly_indicators").append(check_txt);
    }
}

export function resetOverlyIndicators(){
    localStorage[OVERLY] = "{}";
    window.location.reload();
}

export function hideAllOverlyIndicators(){
    for (const name in getStoreDB(OVERLY)) {
        setStore(OVERLY,name,false);
    }
    window.location.reload();
}


window.checkboxChartChange = checkboxChartChange;
window.checkboxChange = checkboxChange;
window["resetSubIndicatorsStorage"] = resetSubIndicatorsStorage;
window["hideAllSubIndicators"] = hideAllSubIndicators;
window["checkboxOverlyChange"] = checkboxOverlyChange;
window["buildOverlyHtml"] = buildOverlyHtml;
window["resetOverlyIndicators"] = resetOverlyIndicators;
window["hideAllOverlyIndicators"] = hideAllOverlyIndicators;

