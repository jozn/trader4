// https://app.quicktype.io/?l=ts

export interface Pokedex {
    major:        ITimeFrameValues;
    medium:       ITimeFrameValues;
    small:        ITimeFrameValues;
    markers:      Marker[];
    score_bull:   ITimeValue[];
    score_bear:   ITimeValue[];
    score_diff:   ITimeValue[];
    major_ma_mom: ITimeValue[];
}

export interface ITimeFrameValues {
    ohlc:           Ohlc[];
    high_line:      any[];
    low_line:       any[];
    markers:        any[];
    ma1:            ITimeValue[];
    ma_mom:         ITimeValue[];
    bull_line:      ITimeValue[];
    bear_line:      ITimeValue[];
    rpi_high:       ITimeValue[];
    rpi_low:        ITimeValue[];
    dmi_plus:       ITimeValue[];
    dmi_minus:      ITimeValue[];
    dmi_diff:       ITimeValue[];
    macd_macd:      ITimeValue[];
    macd_signal:    ITimeValue[];
    macd_histogram: ITimeValue[];
}

export interface ITimeValue {
    time:  number;
    value: number;
}

export interface Ohlc {
    time:  number;
    open:  number;
    high:  number;
    low:   number;
    close: number;
}

export interface Marker {
    time:     number;
    position: string; // "belowBar"
    color:    string;
    shape:    string; // "arrowUp"
    text:     string;
}
