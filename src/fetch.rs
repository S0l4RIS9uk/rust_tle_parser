use chrono::Utc;
use error_chain::error_chain;
use serde::{Deserialize, Serialize};
use std::fs;
use crate::parse::{TLE, parse_tle, split_tle};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
    last_bulk_update: i64,
    tles: Vec<TLE>,
}

impl Cache {
    fn to_file(&self, path: String) -> Result<()> {
        fs::write(
            path,
            serde_json::to_string_pretty(self).expect("Failed to serialise TLE's"),
        )?;
        Ok(())
    }

    async fn update(&mut self) -> Result<&Cache> {
        let updated_tles = split_tle(fetch_tle("GROUP=active".to_string()).await?)
            .iter()
            .map(parse_tle)
            .collect::<Vec<TLE>>();
        for new_tle in updated_tles {
            if let Some(existing_tle) = self
                .tles
                .iter_mut()
                .find(|tle| tle.satellite_number == new_tle.satellite_number)
            {
                *existing_tle = new_tle; // Update existing TLE
            } else {
                self.tles.push(new_tle); // Add new TLE
            }
        }

        // Update the timestamp of the last bulk update
        self.last_bulk_update = Utc::now().timestamp();
        Ok(self)
    }
}

pub async fn load_tle_cache(path: Option<String>) -> Result<Cache> {
    if path.is_none() {
        let cache: Cache = Cache {
            last_bulk_update: Utc::now().timestamp(),
            tles: split_tle(fetch_tle("GROUP=active".to_string()).await?)
                .iter()
                .map(parse_tle)
                .collect(),
        };
        return Ok(cache);
    } else {
        if !fs::metadata(path.as_ref().unwrap()).is_ok() {
            let cache: Cache = Cache {
                last_bulk_update: Utc::now().timestamp(),
                tles: split_tle(fetch_tle("GROUP=active".to_string()).await?)
                    .iter()
                    .map(parse_tle)
                    .collect(),
            };
            return Ok(cache);
        } else {
            let file_contents =
                fs::read_to_string(path.as_ref().unwrap()).expect("Could not read file contents.");
            Ok(serde_json::from_str(&file_contents).expect("Could not deserialise file_contents"))
        }
    }
}

// TODO: Strictly type query options, validate query & cache results.
pub async fn fetch_tle(query: String) -> Result<String> {
    /*     let res = reqwest::get(&format!(
        "https://celestrak.org/NORAD/elements/gp.php?{}&FORMAT=tle",
        query
    ))
    .await?;
    let body = res.text().await?;
    */

    let body = "NOAA 15                 
1 25338U 98030A   24169.93801846  .00000329  00000+0  15393-3 0  9999
2 25338  98.5680 197.0492 0009520 328.5342  31.5268 14.26605440357297
DMSP 5D-3 F16 (USA 172) 
1 28054U 03048A   24169.92781536  .00000136  00000+0  95636-4 0  9999
2 28054  99.0199 176.4030 0008260  55.9567   4.7973 14.14038707 66403
NOAA 18                 
1 28654U 05018A   24169.91608596  .00000284  00000+0  17494-3 0  9996
2 28654  98.8743 246.8749 0015146  64.1200 296.1532 14.13233021983273
METEOSAT-9 (MSG-2)      
1 28912U 05049B   24169.73676108  .00000141  00000+0  00000+0 0  9994
2 28912   7.6876  60.8679 0001564   4.6631 152.0205  1.00278580 67739
EWS-G1 (GOES 13)        
1 29155U 06018A   24169.70220899  .00000099  00000+0  00000+0 0  9992
2 29155   2.6271  85.2887 0081681 255.5015  49.7602  0.99100384 36031
DMSP 5D-3 F17 (USA 191) 
1 29522U 06050A   24169.96455616  .00000149  00000+0  10020-3 0  9994
2 29522  98.7431 180.5248 0009725 344.5590  15.5285 14.14489118909303
FENGYUN 3A              
1 32958U 08026A   24169.87891162  .00000197  00000+0  11449-3 0  9995
2 32958  98.6041 116.9780 0009893  72.7862 287.4396 14.19106051831942
NOAA 19                 
1 33591U 09005A   24169.86755025  .00000269  00000+0  16868-3 0  9993
2 33591  99.0469 225.6740 0012750 291.7508  68.2307 14.13028542791466
GOES 14                 
1 35491U 09033A   24169.87063115 -.00000044  00000+0  00000+0 0  9997
2 35491   0.3405 104.0766 0003981 351.0753  16.3310  1.00272898 54803
DMSP 5D-3 F18 (USA 210) 
1 35951U 09057A   24169.90066725  .00000371  00000+0  21745-3 0  9993
2 35951  98.8134 145.2407 0011446 158.8452 201.3197 14.14139653756480
EWS-G2 (GOES 15)        
1 36411U 10008A   24169.95190812  .00000061  00000+0  00000+0 0  9996
2 36411   0.3033  89.3739 0002071  97.5232 123.7772  1.00268491 52325
COMS 1                  
1 36744U 10032A   24169.92514730 -.00000324  00000+0  00000+0 0  9991
2 36744   2.9389  84.3788 0001067   4.1211 279.3499  1.00271133 43770
FENGYUN 3B              
1 37214U 10059A   24169.92704955  .00000149  00000+0  10120-3 0  9995
2 37214  99.0544 198.0300 0023277  90.7872  30.1057 14.14508141704315
SUOMI NPP               
1 37849U 11061A   24169.88817085  .00000113  00000+0  74122-4 0  9991
2 37849  98.7222 107.8581 0000609  80.2824 279.8421 14.19583987654971
FENGYUN 2F              
1 38049U 12002A   24169.31036115 -.00000338  00000+0  00000+0 0  9994
2 38049   5.5943  73.1345 0002744  81.5422 334.7913  1.00258461 45561
METEOSAT-10 (MSG-3)     
1 38552U 12035B   24169.72770662  .00000014  00000+0  00000+0 0  9996
2 38552   2.9663  60.9963 0000875 326.8630 140.7932  1.00267462 43542
METOP-B                 
1 38771U 12049A   24169.87696835  .00000194  00000+0  10819-3 0  9991
2 38771  98.6271 227.0854 0001606  74.4617 285.6738 14.21554232609655
INSAT-3D                
1 39216U 13038B   24169.86473074 -.00000134  00000+0  00000+0 0  9994
2 39216   0.0452 113.2562 0000615 134.4258  52.3012  1.00270452 39762
FENGYUN 3C              
1 39260U 13052A   24169.94411501  .00000154  00000+0  98595-4 0  9997
2 39260  98.4279 169.5104 0010410 330.9906  29.0689 14.16218350555373
METEOR-M 2              
1 40069U 14037A   24169.94590626  .00000199  00000+0  11095-3 0  9994
2 40069  98.4359 164.1498 0005861   9.2519 350.8766 14.20980663515733
HIMAWARI-8              
1 40267U 14060A   24169.89872993 -.00000253  00000+0  00000+0 0  9996
2 40267   0.0475 149.6520 0000720 310.9908 270.2066  1.00272463 35447
FENGYUN 2G              
1 40367U 14090A   24169.79493730 -.00000283  00000+0  00000+0 0  9990
2 40367   3.7920  81.5312 0002909 109.1256 101.7938  1.00270525 34689
METEOSAT-11 (MSG-4)     
1 40732U 15034A   24169.65792878  .00000086  00000+0  00000+0 0  9995
2 40732   1.3873  69.1260 0001841 153.0686 290.5885  1.00279233   539
ELEKTRO-L 2             
1 41105U 15074A   24169.14424382 -.00000098  00000+0  00000+0 0  9994
2 41105   4.8416  79.4707 0000474 151.4218  72.4700  1.00271898 31186
INSAT-3DR               
1 41752U 16054A   24169.52766076 -.00000056  00000+0  00000+0 0  9996
2 41752   0.1034  94.4140 0010098 174.8679 260.9298  1.00272111 28507
HIMAWARI-9              
1 41836U 16064A   24169.91993774 -.00000253  00000+0  00000+0 0  9999
2 41836   0.0295 232.0778 0001327 238.5022 268.0107  1.00270227 27895
GOES 16                 
1 41866U 16071A   24169.85396544 -.00000226  00000+0  00000+0 0  9995
2 41866   0.0697 254.6263 0000473 177.8557  66.3349  1.00272294 27794
FENGYUN 4A              
1 41882U 16077A   24169.86473074 -.00000180  00000+0  00000+0 0  9993
2 41882   0.4599  85.6417 0008321 344.3654 234.8810  1.00269152 27614
CYGFM05                 
1 41884U 16078A   24169.73486065  .00009459  00000+0  34784-3 0  9993
2 41884  34.9551 297.6192 0011797 204.0108 156.0058 15.27049596416161
CYGFM04                 
1 41885U 16078B   24169.65687045  .00009067  00000+0  32451-3 0  9990
2 41885  34.9436 282.1784 0009394 227.0613 132.9314 15.27970636416241
CYGFM02                 
1 41886U 16078C   24169.68149277  .00009475  00000+0  33548-3 0  9995
2 41886  34.9530 284.2776 0012357 214.2290 145.7630 15.28283305416337
CYGFM01                 
1 41887U 16078D   24169.65202250  .00025680  00000+0  94505-3 0  9990
2 41887  34.9497 301.9852 0011243 191.2472 168.7993 15.26878977416083
CYGFM08                 
1 41888U 16078E   24169.66910763  .00009176  00000+0  32999-3 0  9998
2 41888  34.9504 290.0455 0012359 216.4250 143.5625 15.27778192416265
CYGFM07                 
1 41890U 16078G   24169.64620983  .00009202  00000+0  32808-3 0  9990
2 41890  34.9505 281.6446 0009162 226.0498 133.9462 15.28094082416255
CYGFM03                 
1 41891U 16078H   24169.68120266  .00009435  00000+0  33715-3 0  9990
2 41891  34.9506 282.6100 0010330 209.0546 150.9595 15.28004783416243
FENGYUN 3D              
1 43010U 17072A   24169.94071995  .00000140  00000+0  88187-4 0  9991
2 43010  98.9107 124.5996 0001568  19.7843 340.3393 14.19358353341524
NOAA 20 (JPSS-1)        
1 43013U 17073A   24169.90584172  .00000089  00000+0  62647-4 0  9992
2 43013  98.7065 107.8155 0000956 138.6960 221.4288 14.19597428341003
GOES 17                 
1 43226U 18022A   24169.41457468 -.00000069  00000+0  00000+0 0  9995
2 43226   0.0482 155.2292 0000888 317.1460 198.3195  1.00272114 23105
FENGYUN 2H              
1 43491U 18050A   24169.82340962 -.00000107  00000+0  00000+0 0  9998
2 43491   1.1443  87.2792 0001725  32.8145 162.1468  1.00269727 22128
METOP-C                 
1 43689U 18087A   24169.91556426  .00000190  00000+0  10690-3 0  9992
2 43689  98.7233 229.1839 0003163  90.8753 269.2787 14.21475293291228
GEO-KOMPSAT-2A          
1 43823U 18100A   24169.55604927 -.00000322  00000+0  00000+0 0  9996
2 43823   0.0460 149.2508 0002459 299.0011 146.4783  1.00272371 20315
METEOR-M2 2             
1 44387U 19038A   24169.92344879  .00000059  00000+0  45474-4 0  9996
2 44387  98.8265 139.1500 0001105  18.5176 341.6042 14.23894557257495
ARKTIKA-M 1             
1 47719U 21016A   24169.25177733  .00000188  00000+0  00000+0 0  9990
2 47719  63.1394 151.7679 6941537 268.6991  17.9346  2.00610662 24142
FENGYUN 3E              
1 49008U 21062A   24169.91121447  .00000040  00000+0  39414-4 0  9991
2 49008  98.7315 171.1805 0002424  55.3238 304.8166 14.19759233153097
GOES 18                 
1 51850U 22021A   24169.79702095  .00000119  00000+0  00000+0 0  9996
2 51850   0.0439 161.0212 0000534 271.5836 343.8503  1.00271024  8454
NOAA 21 (JPSS-2)        
1 54234U 22150A   24169.87056381  .00000119  00000+0  77534-4 0  9998
2 54234  98.7518 107.7563 0001752 107.8423 252.2945 14.19527293 83077
METEOSAT-12 (MTG-I1)    
1 54743U 22170C   24169.89146657  .00000014  00000+0  00000+0 0  9996
2 54743   0.5127  41.3734 0001850  42.5354 143.1564  1.00274140  5669
TIANMU-1 03             
1 55973U 23039A   24169.91489940  .00008689  00000+0  36661-3 0  9997
2 55973  97.4682 320.5271 0009192 154.7813 205.3875 15.23494043 68961
TIANMU-1 04             
1 55974U 23039B   24169.89781334  .00009132  00000+0  38411-3 0  9996
2 55974  97.4604 320.0715 0009891 152.5846 207.5914 15.23577504 68953
TIANMU-1 05             
1 55975U 23039C   24169.94727601  .00008629  00000+0  36330-3 0  9998
2 55975  97.4652 320.4480 0008351 160.8369 199.3183 15.23578097 68956
TIANMU-1 06             
1 55976U 23039D   24169.93116707  .00008249  00000+0  34809-3 0  9994
2 55976  97.4666 320.5491 0008036 166.3462 193.7994 15.23516498 68955
METEOR-M2 3             
1 57166U 23091A   24169.88839421  .00000029  00000+0  31722-4 0  9994
2 57166  98.7125 224.5558 0005004 120.7627 239.4044 14.23899347 50724
TIANMU-1 07             
1 57399U 23101A   24169.88699475  .00005788  00000+0  29467-3 0  9996
2 57399  97.3634 227.0200 0005499 168.1634 191.9730 15.17171958 50534
TIANMU-1 08             
1 57400U 23101B   24169.92231860  .00005872  00000+0  29868-3 0  9993
2 57400  97.3622 226.9273 0007506 178.6603 181.4652 15.17183475 50539
TIANMU-1 09             
1 57401U 23101C   24169.90522165  .00006459  00000+0  32837-3 0  9995
2 57401  97.3578 226.7529 0008044 175.0882 185.0432 15.17163356 50532
TIANMU-1 10             
1 57402U 23101D   24169.93735460  .00005649  00000+0  28767-3 0  9990
2 57402  97.3657 227.0600 0010232 176.0174 184.1143 15.17142917 50531
FENGYUN 3F              
1 57490U 23111A   24169.93600678  .00000117  00000+0  75903-4 0  9990
2 57490  98.7695 239.0605 0002014  92.2808 267.8598 14.19826896 45370
ARKTIKA-M 2             
1 58584U 23198A   24169.48278552  .00000150  00000+0  00000+0 0  9990
2 58584  63.2417 235.5556 6979408 269.9021  17.2518  2.00611967  3681
TIANMU-1 11             
1 58645U 23205A   24169.91170139  .00004968  00000+0  26813-3 0  9992
2 58645  97.4029 194.9823 0012635  49.4406 310.7924 15.15123692 26628
TIANMU-1 12             
1 58646U 23205B   24169.86117996  .00005425  00000+0  29226-3 0  9999
2 58646  97.4016 194.8642 0010996  43.9435 316.2670 15.15160970 26609
TIANMU-1 13             
1 58647U 23205C   24169.95992661  .00005955  00000+0  32059-3 0  9996
2 58647  97.4044 194.9546 0010405  30.0619 330.1208 15.15155262 26612
TIANMU-1 14             
1 58648U 23205D   24169.87775001  .00005956  00000+0  32054-3 0  9991
2 58648  97.3971 194.7387 0010926  30.4838 329.7028 15.15158776 26603
TIANMU-1 19             
1 58660U 23208A   24169.88477053  .00005869  00000+0  31661-3 0  9991
2 58660  97.4111 282.2568 0009078  12.0706 348.0742 15.15098466 26273
TIANMU-1 20             
1 58661U 23208B   24169.88795868  .00005820  00000+0  30695-3 0  9992
2 58661  97.4039 282.2338 0010138  21.3384 338.8270 15.15894832 26287
TIANMU-1 21             
1 58662U 23208C   24169.88054556  .00006521  00000+0  33495-3 0  9995
2 58662  97.4078 282.4863 0009765  21.6342 338.5302 15.16784500 26292
TIANMU-1 22             
1 58663U 23208D   24169.90123721  .00006016  00000+0  32482-3 0  9994
2 58663  97.4098 282.3488 0005834  39.6894 320.4765 15.15077087 26287
TIANMU-1 15             
1 58700U 24004A   24169.94171269  .00005398  00000+0  29166-3 0  9992
2 58700  97.4620 351.0326 0011081  56.1325 304.0960 15.15062670 24880
TIANMU-1 16             
1 58701U 24004B   24169.92459475  .00006026  00000+0  32487-3 0  9998
2 58701  97.4547 350.8952 0011126  61.2807 298.9542 15.15097364 24888
TIANMU-1 17             
1 58702U 24004C   24169.90951200  .00005886  00000+0  31775-3 0  9992
2 58702  97.4592 351.0102 0008924  57.8399 302.3698 15.15073775 24879
TIANMU-1 18             
1 58703U 24004D   24169.89274914  .00005193  00000+0  28058-3 0  9992
2 58703  97.4608 351.0655 0008026  50.2308 309.9630 15.15105731 24878
INSAT-3DS               
1 58990U 24033A   24166.45222727 -.00000153  00000+0  00000+0 0  9991
2 58990   0.0993  93.3619 0011635 357.7741  57.9093  1.00271183   944
METEOR-M2 4             
1 59051U 24039A   24169.90863199  .00000061  00000+0  46727-4 0  9991
2 59051  98.5975 131.7840 0008077 117.8947 242.3050 14.22232617 15594"
        .to_string();
    Ok(body)
}