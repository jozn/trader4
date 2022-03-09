import "./types";
import * as LightweightCharts from 'lightweight-charts' ;
// import * as sub from './sub_iindicators' ;
// import * as sub from './sub_iindicators' ;
import {BarSeriesPartialOptions, IChartApi, ISeriesApi, SeriesMarker, Time} from "lightweight-charts";

(function() {
    // run();
    // Some wierd issues without timeout. (some global variables is not set).
    setTimeout(run,50);
})();

var width = 900;

function run() {
    console.log("------ hi there ----");
    makeBarChart();
    var c =  $$("sdf");
    // sub.scoreChart(1,2);
}

function $$(id) {
    return document.getElementById(id);
}


// Make Bar Chart

function getChartCfg(width,height) {
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


function makeBarChart() {
    width =  window.innerWidth * 0.98;
    var chartMajorEl = document.getElementById("chart_major");
    var chartMediumEL = document.getElementById("chart_medium");
    var chartSmallEL = document.getElementById("chart_small");

    var major_check_btn = document.getElementById("major_check_btn");
    var medium_check_btn = document.getElementById("medium_check_btn");
    var small_check_btn = document.getElementById("small_check_btn");

    var chart2El = document.getElementById("chart2");
    var chart3El = document.getElementById("chart3");
    var chart4El = document.getElementById("chart4");

    var jsonText = document.getElementById("json_data").innerText;
    var jsonData = JSON.parse(jsonText);
    var jd = jsonData;

    // Create main Bar charts
    var chart_major = buildBarChart({black: false, el: chartMajorEl, height: width/8, ohlc: jd.major.ohlc});
    var chart_medium = buildBarChart({black: true, el: chartMediumEL, height: width/5, ohlc: jd.medium.ohlc,markers: jd.markers});
    var chart_small = buildBarChart({black: false, el: chartSmallEL, height: width/6, ohlc: jd.small.ohlc,markers: jd.markers});

    // Syncs
    syncCharts(chart_major,chart_medium);
    syncCharts(chart_medium,chart_small);

    // Add bull/bear channel to main bars
    trendChannelChart(chart_major,jd.major);
    trendChannelChart(chart_medium,jd.medium);
    trendChannelChart(chart_small,jd.small);

    // RPI indicaotr
    rpiOverIndicator(chart_major,jd.major);
    rpiOverIndicator(chart_medium,jd.medium);
    rpiOverIndicator(chart_small,jd.small);

    // MA
    simpleLineOver(chart_major,jd.major.ma1);
    simpleLineOver(chart_medium,jd.major.ma1);
    simpleLineOver(chart_small,jd.major.ma1);

    ///////////////////////////  Dynamic Sub Charts ////////////////////////
    // Sub Indicators
    let el_macd = makeNextIndi("macd",true,true);
    // var macd1_el = document.getElementById("macd1");
    // macd_chart1 = macdChart(macd1_el,jd.medium);
    // var macd_chart1 = macdChart(macd1_el,jd.small);
    var macd_chart1 = macdChart(el_macd,jd.small);
    syncCharts(chart_medium,macd_chart1);

    // Score
    // var tscore_el = document.getElementById("tscore");
    var tscore_el = makeNextIndi("tscore",true,true);
    var tscore_chart = scoreChart(tscore_el,jd);
    syncCharts(chart_medium,tscore_chart);

    // MDI
    // var medium_dmi_el = document.getElementById("medium_dmi");
    // var medium_dmi_el = document.getElementById("medium_dmi");
    var medium_dmi_el = makeNextIndi("medium_dmi",false,false);

    // var jdm = jd.major;
    // var jdd = jd.medium;
    var medium_dmi = mdi(medium_dmi_el,jd.medium);
    syncCharts(chart_medium,medium_dmi);


    // MA Mom
    // var ma_mom_el = document.getElementById("ma_mom");
    var ma_mom_el = makeNextIndi("ma_mom",false,true);
    var ma_mom_chart = maMomChart(ma_mom_el,jd);
    syncCharts(chart_medium,ma_mom_chart);

}

function buildBarChart(p: {el: HTMLElement,height:number,black:boolean,ohlc:any,markers?: SeriesMarker<Time>[]}) :IChartApi {
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

function trendChannelChart(chart: IChartApi, d) {
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

function rpiOverIndicator(chart: IChartApi, d) {
    var highLine = chart.addLineSeries({
        color: 'rgb(158,162,129)',
        lineWidth: 1,
    });
    highLine.setData(d.rpi_high);

    var lowLine = chart.addLineSeries({
        color: 'rgb(103,100,100)',
        lineWidth: 1,
    });
    lowLine.setData(d.rpi_low);
}

// Simple
function simpleLineOver(chart: IChartApi, d, color?: string) {
    if (color == undefined){
        color = 'rgb(11,77,229)';
    }
    var line = chart.addLineSeries({
        color: color,
        lineWidth: 1,
    });
    line.setData(d);
}

// Sub indicators
function macdChart(el,d) {
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

function scoreChart(el,d) {
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

function mdi(el,d) {
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

function maMomChart(el,d) {
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
    scoreBull.setData(d.major_ma_mom);

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

const takenIds:any = {};
var top_indicators = "#top_indicators";
function makeNextIndi(name:string,visible:boolean,topHolder:boolean){
    if (takenIds === undefined){
        // takenIds ={};
    }
    if(takenIds[name] != undefined) {
        console.log("indiactor name: {}" + name + " is already taken. abort");
        return
    }
    takenIds[name] = name;

    var checked_attr = "checked";
    if(visible == false){
        checked_attr = "";
    }

    // jQuery
    let check_txt = `<input type="checkbox" ${checked_attr} id="btn_${name}" onchange="checkboxChange(this)" class="checkbox"  > ${name} </input>`;
    $("form").append(check_txt);

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
function syncCharts(chart1: IChartApi,chart2: IChartApi){
    chart1.timeScale().subscribeVisibleTimeRangeChange((t) => chart2.timeScale().setVisibleRange(t))
}

function checkboxChange(th: HTMLElement){
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

    for (const name in takenIds) {
        let el = $$("btn_"+name);
        let chart_el = $$("chart_"+name);
        if(el.checked) {
            chart_el.style.display = "block";
        } else {
            chart_el.style.display = "none";
        }
    }
}