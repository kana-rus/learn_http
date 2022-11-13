pub(crate) struct Date {
    sec:   u8,
    min:   u8,
    hour:  u8,
    day:   Day,
    date:  u8,
    month: Month,
    year:  u16,
}
enum Day {
    Monday    = 1,
    Tueesday  = 2,
    Wednesday = 3,
    Thursday  = 4,
    Friday    = 5,
    Saturday  = 6,
    Sunday    = 7,
}
enum Month {
    January   = 1,
    Febrary   = 2,
    March     = 3,
    April     = 4,
    May       = 5,
    June      = 6,
    July      = 7,
    August    = 8,
    September = 9,
    October   = 10,
    November  = 11,
    December  = 12,
}