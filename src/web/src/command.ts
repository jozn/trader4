import * as fn from "./funcs";

(function() {
    setTimeout(run,50);
})();

function run() {
    console.log("------ Running run fn ----");
    makeBarChart();
    fn.checkboxChange(null);
}

function makeBarChart() {
    var width =  window.innerWidth * 0.98;
    var chartMajorEl = document.getElementById("chart_major");
    var chartMediumEL = document.getElementById("chart_medium");
    var chartSmallEL = document.getElementById("chart_small");

    var jsonText = document.getElementById("json_data").innerText;
    var jd = JSON.parse(jsonText);

    // Create main Bar charts
    var chart_major = fn.buildBarChart({black: false, el: chartMajorEl, height: width/8, ohlc: jd.major.ohlc});
    var chart_medium = fn.buildBarChart({black: true, el: chartMediumEL, height: width/5, ohlc: jd.medium.ohlc,markers: jd.markers});
    var chart_small = fn.buildBarChart({black: false, el: chartSmallEL, height: width/6, ohlc: jd.small.ohlc,markers: jd.markers});
    // Syncs
    fn.syncCharts(chart_major,chart_medium);
    fn.syncCharts(chart_medium,chart_small);

    // Add bull/bear channel to main bars
    fn.trendChannelChart(chart_major,jd.major);
    fn.trendChannelChart(chart_medium,jd.medium);
    fn.trendChannelChart(chart_small,jd.small);

    // RPI indicaotr
    fn.rpiOverIndicator(chart_major,jd.major);
    fn.rpiOverIndicator(chart_medium,jd.medium);
    fn.rpiOverIndicator(chart_small,jd.small);

    // MA
    fn.simpleLineOver(chart_major,jd.major.ma1);
    fn.simpleLineOver(chart_medium,jd.major.ma1);
    fn.simpleLineOver(chart_small,jd.major.ma1);

    ///////////////////////////  Dynamic Sub Charts ////////////////////////
    // Sub Indicators
    let el_macd = fn.makeNextIndi("macd",true,true);
    var macd_chart1 = fn.macdChart(el_macd,jd.small);
    fn.syncCharts(chart_medium,macd_chart1);

    // Score
    var tscore_el = fn.makeNextIndi("tscore",true,true);
    var tscore_chart = fn.scoreChart(tscore_el,jd);
    fn.syncCharts(chart_medium,tscore_chart);

    // MDI
    var medium_dmi_el = fn.makeNextIndi("medium_dmi",true,false);
    var medium_dmi = fn.mdi(medium_dmi_el,jd.medium);
    fn.syncCharts(chart_medium,medium_dmi);

    // MA Mom
    var ma_mom_el = fn.makeNextIndi("ma_mom",true,true);
    var ma_mom_chart = fn.maMomChart(ma_mom_el,jd);
    fn.syncCharts(chart_medium,ma_mom_chart);

}
