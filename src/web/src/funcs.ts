import "./types";
import * as LightweightCharts from 'lightweight-charts' ;
import {BarSeriesPartialOptions, IChartApi, ISeriesApi, SeriesMarker, Time} from "lightweight-charts";

var width = 1600;

(function() {
    width =  window.innerWidth * 0.98;
})();
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
export function simpleLineOver(chart: IChartApi, d, color?: string) {
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
export function makeNextIndi(name:string,visible:boolean,topHolder:boolean){
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
export function syncCharts(chart1: IChartApi,chart2: IChartApi){
    chart1.timeScale().subscribeVisibleTimeRangeChange((t) => chart2.timeScale().setVisibleRange(t))
}

export function checkboxChange(th: HTMLElement){
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
window.checkboxChange = checkboxChange;

