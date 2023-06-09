impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let total = (1<<n)-1;
        let mut graph = vec![vec![];41];
        for i in 0..n {
            for &hat in &hats[i]{
                graph[hat as usize].push(i as i32);
            }
        }
        let mut dp = vec![vec![-1;total+1];41];
        //dp[i][j] = i번 모자까지 j 마스크 방문했을 때 방법의 수
        solve(1,0,&mut dp ,&graph)
    }
}
const MOD:i32 = 1_000_000_007;
fn solve(hidx:usize, mask:usize, memo:&mut Vec<Vec<i32>>, graph:&Vec<Vec<i32>>)->i32{
    if mask==memo[0].len()-1{
        return 1 //모든 사람이 다씀
    }
    if hidx>40{
        return 0 //모자 개수 넘어감
    }
    if memo[hidx][mask]!=-1{
        return memo[hidx][mask]
    }
    let mut res = solve(hidx+1, mask,memo,graph); //현재 모자 아무도 안씀
    for &p in &graph[hidx]{  //현재 모자 가능한 사람
        if (mask>>p)&1>0{ //다른 모자 안써본 사람
            continue
        }
        res = (res+solve(hidx+1,mask|(1<<p), memo,graph))%MOD;
    }
    memo[hidx][mask]=res;
    res
}